pub mod content;

use bevy::{prelude::*, sprite::Anchor};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision::{intersections_with, ColliderBundle},
    subjects::{state::UpdateSubjectState, DamageSubjects, Subject},
    units::{Frontlines, Health},
    AppState, Kingdom, GRAVITY_ACCELERATION, PX_PER_METER,
};

pub const MAX_ARROW_DEPTH: f32 = 0.125;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponAssets>()
            .add_event::<ShotEvent>()
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_arrows))
            .add_system(swing_swords.label(DamageSubjects))
            .add_system(tick_bows)
            .add_system(shoot_bows.after(tick_bows).after(UpdateSubjectState))
            .add_system(accelerate_arrows.after(shoot_bows))
            .add_system(rotate_arrows.after(accelerate_arrows))
            .add_system(
                collide_arrows
                    .label(DamageSubjects)
                    .after(accelerate_arrows)
                    .after(swing_swords),
            )
            .add_system(despawn_lifetimes.after(shoot_bows));
    }
}

#[derive(Resource)]
struct WeaponAssets {
    arrow_sprite: Handle<Image>,
    arrow_ground_hit_sound: Handle<AudioSource>,
    bow_shot_sound: Handle<AudioSource>,
}

impl FromWorld for WeaponAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.resource();
        WeaponAssets {
            arrow_sprite: asset_server.load("sprites/elven_arrow.png"),
            arrow_ground_hit_sound: asset_server.load("sounds/arrow_ground_hit.wav"),
            bow_shot_sound: asset_server.load("sounds/bow_shot.wav"),
        }
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
    pub damage: u32,
    pub range: f32,
    pub spread: f32,
    pub speed: f32,
    pub timer: Timer,
}

impl Bow {
    pub fn new(damage: u32, range: f32, spread: f32, speed: f32, recharge_seconds: f32) -> Self {
        let mut timer = Timer::from_seconds(recharge_seconds, TimerMode::Once);
        timer.set_elapsed(timer.duration());
        Self {
            damage,
            range,
            spread,
            speed,
            timer,
        }
    }
}

#[derive(Component)]
struct Arrow {
    damage: u32,
    is_grounded: bool,
}

impl Arrow {
    pub fn new(damage: u32) -> Self {
        Self {
            damage,
            is_grounded: false,
        }
    }
}

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

fn swing_swords(
    sword_query: Query<(Entity, &Kingdom), With<Sword>>,
    mut health_query: Query<(&Kingdom, &mut Health)>,
    context: Res<RapierContext>,
) {
    for (sword_entity, sword_kingdom) in &sword_query {
        for health_entity in intersections_with(sword_entity, &context) {
            let Ok((health_kingdom, mut health)) = health_query.get_mut(health_entity) else {
                continue;
            };

            if health_kingdom != sword_kingdom {
                health.damage(1);
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
    mut bow_query: Query<(&Transform, &Kingdom, &mut Bow)>,
    target_query: Query<(&Transform, &Velocity), With<Subject>>,
    frontlines: Res<Frontlines>,
    assets: Res<WeaponAssets>,
    audio: Res<Audio>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let Ok((bow_transform, kingdom, mut bow)) = bow_query.get_mut(event.bow_entity) else {
            continue;
        };
        let frontline_entity = match kingdom {
            Kingdom::Elven => frontlines.monster.entity,
            Kingdom::Monster => frontlines.elven.entity,
        };
        let Some(target_entity) = frontline_entity else {
            continue;
        };
        let Ok((target_transform, target_velocity)) = target_query.get(target_entity) else {
            continue;
        };
        bow.timer.reset();

        let position = match *kingdom {
            Kingdom::Elven => bow_transform.translation + Vec3::new(0.4, 0.0, 0.0),
            Kingdom::Monster => bow_transform.translation + Vec3::new(-0.4, 0.0, 0.0),
        };

        let diff = target_transform.translation - position;
        let random_offset = 0.85 + 0.3 * fastrand::f32();
        let speed = bow.speed * random_offset;
        let velocity_x = speed * diff.x.signum();
        let relative_velocity_x = velocity_x - target_velocity.linvel.x;
        let random_offset = (1.0 - bow.spread / 4.0) + bow.spread * fastrand::f32();
        // TODO: this doesn't work when the target runs away faster than the arrow
        let flight_time = (diff.x / relative_velocity_x) * random_offset;
        // TODO: this doesn't take vertical velocity into account for prediction
        let velocity_y = diff.y / flight_time + GRAVITY_ACCELERATION * flight_time / 2.0;
        let velocity = Vec2::new(velocity_x, velocity_y);

        spawn_arrow(
            position,
            velocity,
            *kingdom,
            bow.damage,
            &assets,
            &mut commands,
        );

        let sound = assets.bow_shot_sound.clone();
        audio.play(sound).with_volume(0.5);
    }
}

fn despawn_arrows(query: Query<Entity, With<Arrow>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn accelerate_arrows(mut query: Query<(&mut Velocity, &Arrow)>, time: Res<Time>) {
    for (mut velocity, arrow) in &mut query {
        if !arrow.is_grounded {
            velocity.linvel.y -= GRAVITY_ACCELERATION * time.delta_seconds();
        }
    }
}

fn rotate_arrows(mut query: Query<(&mut Transform, &Velocity), With<Arrow>>) {
    for (mut transform, velocity) in &mut query {
        if velocity.linvel != Vec2::ZERO {
            let rotation = Vec2::angle_between(Vec2::X, velocity.linvel);
            transform.rotation = Quat::from_rotation_z(rotation);
        }
    }
}

fn collide_arrows(
    mut arrow_query: Query<(Entity, &mut Transform, &mut Velocity, &Kingdom, &mut Arrow)>,
    mut health_query: Query<(&Kingdom, &mut Health)>,
    context: Res<RapierContext>,
    weapon_assets: Res<WeaponAssets>,
    audio: Res<Audio>,
    mut commands: Commands,
) {
    for (arrow_entity, mut transform, mut velocity, arrow_kingdom, mut arrow) in &mut arrow_query {
        if arrow.is_grounded {
            continue;
        }

        if transform.translation.y <= -MAX_ARROW_DEPTH {
            transform.translation.y = -MAX_ARROW_DEPTH;
            velocity.linvel = Vec2::ZERO;
            arrow.is_grounded = true;

            let sound = weapon_assets.arrow_ground_hit_sound.clone();
            audio.play(sound).with_volume(0.15);

            continue;
        }

        for health_entity in intersections_with(arrow_entity, &context) {
            let Ok((health_kingdom, mut health)) = health_query.get_mut(health_entity) else {
                continue;
            };

            if !health.is_dead() && health_kingdom != arrow_kingdom {
                health.damage(arrow.damage);
                commands.entity(arrow_entity).despawn_recursive();
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

fn spawn_arrow(
    position: Vec3,
    velocity: Vec2,
    kingdom: Kingdom,
    damage: u32,
    assets: &WeaponAssets,
    commands: &mut Commands,
) {
    let root = (
        Name::new("Arrow"),
        SpatialBundle::from_transform(Transform::from_translation(position)),
        RigidBody::KinematicVelocityBased,
        ColliderBundle::new(Collider::ball(0.05)),
        Velocity::linear(velocity),
        Lifetime::new(20.0),
        kingdom,
        Arrow::new(damage),
    );
    let sprite = SpriteBundle {
        texture: assets.arrow_sprite.clone(),
        // texture_atlas: texture_atlases.add(texture_atlas),
        sprite: Sprite {
            anchor: Anchor::CenterRight,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::splat(1.0 / PX_PER_METER),
            ..default()
        },
        ..default()
    };
    commands.spawn(root).with_children(|builder| {
        builder.spawn(sprite);
    });
}
