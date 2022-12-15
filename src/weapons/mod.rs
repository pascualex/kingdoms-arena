pub mod content;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision::{intersections_with, ColliderBundle},
    palette,
    subjects::{despawn_dead_subjects, states::UpdateSubjectState, Frontlines, Health, Subject},
    Kingdom, GRAVITY_ACCELERATION,
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShotEvent>()
            .add_system(swing_subject_swords.before(despawn_dead_subjects))
            .add_system(tick_bows)
            .add_system(shoot_bows.after(tick_bows).after(UpdateSubjectState))
            .add_system(set_arrow_velocities.after(shoot_bows))
            .add_system(
                collide_arrows
                    .after(set_arrow_velocities)
                    .after(swing_subject_swords)
                    .before(despawn_dead_subjects),
            )
            .add_system(despawn_lifetimes.after(shoot_bows));
    }
}

pub struct ShotEvent {
    bow_entity: Entity,
}

impl ShotEvent {
    pub fn new(bow_entity: Entity) -> Self {
        Self { bow_entity }
    }
}

#[derive(Component)]
pub struct Sword;

#[derive(Component)]
pub struct Bow {
    pub range: f32,
    pub timer: Timer,
}

impl Bow {
    pub fn new(range: f32, fire_rate: f32) -> Self {
        Self {
            range,
            timer: Timer::from_seconds(1.0 / fire_rate, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct Arrow;

#[derive(Component)]
pub struct Lifetime {
    timer: Timer,
}

impl Lifetime {
    fn new(lifetime_seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(lifetime_seconds, TimerMode::Once),
        }
    }
}

fn swing_subject_swords(
    sword_query: Query<(Entity, &Kingdom), (With<Subject>, With<Sword>)>,
    mut health_query: Query<(&Kingdom, &mut Health), With<Subject>>,
    context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (sword_entity, sword_kingdom) in &sword_query {
        for health_entity in intersections_with(sword_entity, &context) {
            let Ok((health_kingdom, mut health)) = health_query.get_mut(health_entity) else {
                continue;
            };

            if health_kingdom != sword_kingdom {
                health.damage(1);

                let sound = asset_server.load("sounds/human_death.wav");
                audio.play(sound).with_volume(0.2);
            }
        }
    }
}

fn tick_bows(mut query: Query<&mut Bow>, time: Res<Time>) {
    for mut bow in &mut query {
        bow.timer.tick(time.delta());
    }
}

fn shoot_bows(
    mut events: EventReader<ShotEvent>,
    bow_query: Query<(&Transform, &Kingdom)>,
    target_query: Query<(&Transform, &Velocity), With<Subject>>,
    frontlines: Res<Frontlines>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let Ok((bow_transform, kingdom)) = bow_query.get(event.bow_entity) else {
            continue;
        };
        let frontline_entity = match kingdom {
            Kingdom::Human => frontlines.monster.entity,
            Kingdom::Monster => frontlines.human.entity,
        };
        let Some(target_entity) = frontline_entity else {
            continue;
        };
        let Ok((target_transform, target_velocity)) = target_query.get(target_entity) else {
            continue;
        };
        shoot_arrow(
            bow_transform.translation,
            target_transform.translation,
            target_velocity.linvel,
            *kingdom,
            &audio,
            &asset_server,
            &mut commands,
        );
    }
}

fn shoot_arrow(
    bow_position: Vec3,
    target_position: Vec3,
    target_velocity: Vec2,
    kingdom: Kingdom,
    audio: &Audio,
    asset_server: &AssetServer,
    commands: &mut Commands,
) {
    let diff = target_position - bow_position;
    let random_offset = 0.85 + 0.3 * fastrand::f32();
    let speed = 10.0 * random_offset;
    let velocity_x = speed * diff.x.signum();
    let relative_velocity_x = velocity_x - target_velocity.x;
    let random_offset = 0.75 + 0.75 * fastrand::f32();
    // TODO: this doesn't work when the target runs away faster than the arrow
    let flight_time = (diff.x / relative_velocity_x) * random_offset;
    // TODO: this doesn't take vertical velocity into account for prediction
    let velocity_y = diff.y / flight_time + GRAVITY_ACCELERATION * flight_time / 2.0;

    commands.spawn((
        Name::new("Arrow"),
        SpriteBundle {
            sprite: Sprite {
                color: palette::DARK_YELLOW,
                custom_size: Some(Vec2::new(0.1, 0.1)),
                ..default()
            },
            transform: Transform::from_translation(bow_position),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        ColliderBundle::new(Collider::cuboid(0.05, 0.05)),
        Velocity::linear(Vec2::new(velocity_x, velocity_y)),
        Lifetime::new(20.0),
        kingdom,
        Arrow,
    ));

    let sound = asset_server.load("sounds/bow_shot.wav");
    audio.play(sound).with_volume(0.5);
}

fn set_arrow_velocities(mut query: Query<&mut Velocity, With<Arrow>>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.linvel.y -= GRAVITY_ACCELERATION * time.delta_seconds();
    }
}

fn collide_arrows(
    mut arrow_query: Query<(Entity, &mut Transform, &mut Velocity, &Kingdom), With<Arrow>>,
    mut subject_query: Query<(&Kingdom, &mut Health), With<Subject>>,
    context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (arrow_entity, mut transform, mut velocity, arrow_kingdom) in &mut arrow_query {
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            velocity.linvel = Vec2::ZERO;

            commands.entity(arrow_entity).remove::<Arrow>();

            let sound = asset_server.load("sounds/arrow_ground_hit.wav");
            audio.play(sound).with_volume(0.15);

            continue;
        }

        for subject_entity in intersections_with(arrow_entity, &context) {
            let Ok((subject_kingdom, mut health)) = subject_query.get_mut(subject_entity) else {
                continue;
            };

            if !health.is_dead() && subject_kingdom != arrow_kingdom {
                health.damage(1);
                commands.entity(arrow_entity).despawn_recursive();

                let sound = asset_server.load("sounds/monster_death.wav");
                audio.play(sound).with_volume(0.2);

                break;
            }
        }
    }
}

fn despawn_lifetimes(
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
