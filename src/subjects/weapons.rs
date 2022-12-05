use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::{intersections_with, ColliderBundle},
    palette,
    subjects::{
        despawn_dead_subjects, states::UpdateSubjectState, Frontlines, Health, ShootingState,
        Subject,
    },
    Kingdom, GRAVITY_ACCELERATION,
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(swing_subject_swords.before(despawn_dead_subjects))
            .add_system(recharge_bows)
            .add_system(
                shoot_subject_bows
                    .after(recharge_bows)
                    .after(UpdateSubjectState),
            )
            .add_system(set_arrow_velocities.after(shoot_subject_bows))
            .add_system(
                collide_arrows
                    .after(set_arrow_velocities)
                    .after(swing_subject_swords)
                    .before(despawn_dead_subjects),
            )
            .add_system(despawn_lifetimes.after(shoot_subject_bows));
    }
}

#[derive(Component)]
pub struct Sword;

#[derive(Component)]
pub struct Bow {
    timer: Timer,
}

impl Bow {
    pub fn new(cooldown_seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(cooldown_seconds, TimerMode::Once),
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

fn recharge_bows(mut query: Query<&mut Bow>, time: Res<Time>) {
    for mut bow in &mut query {
        bow.timer.tick(time.delta());
    }
}

fn shoot_subject_bows(
    mut bow_query: Query<(&Transform, &Kingdom, &mut Bow), (With<Subject>, With<ShootingState>)>,
    target_query: Query<&Transform, With<Subject>>,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (bow_transform, kingdom, mut bow) in &mut bow_query {
        if !bow.timer.finished() {
            continue;
        }
        let target_entity = match kingdom {
            Kingdom::Human => frontlines.monster.entity,
            Kingdom::Monster => frontlines.human.entity,
        };
        let Some(target_entity) = target_entity else {
            continue;
        };
        let target_transform = target_query.get(target_entity).unwrap();
        let diff = target_transform.translation - bow_transform.translation;
        let velocity_x = 10.0 * diff.x.signum();
        let flight_time = diff.x / velocity_x;
        let velocity_y = diff.y / flight_time + GRAVITY_ACCELERATION * flight_time.powi(2) / 2.0;
        bow.timer.reset();
        commands.spawn((
            Name::new("Arrow"),
            SpriteBundle {
                sprite: Sprite {
                    color: palette::DARK_YELLOW,
                    custom_size: Some(Vec2::new(0.1, 0.1)),
                    ..default()
                },
                transform: Transform::from_translation(bow_transform.translation),
                ..default()
            },
            ColliderBundle::kinematic(Collider::cuboid(0.05, 0.05)),
            Velocity::linear(Vec2::new(velocity_x, velocity_y)),
            Lifetime::new(60.0),
            kingdom.clone(),
            Arrow,
        ));
    }
}

fn set_arrow_velocities(mut query: Query<&mut Velocity, With<Arrow>>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.linvel.y -= GRAVITY_ACCELERATION * time.delta_seconds();
    }
}

fn collide_arrows(
    mut arrow_query: Query<(Entity, &Transform, &mut Velocity, &Kingdom), With<Arrow>>,
    mut subject_query: Query<(&Kingdom, &mut Health), With<Subject>>,
    context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (arrow_entity, transform, mut velocity, arrow_kingdom) in &mut arrow_query {
        if transform.translation.y <= 0.0 {
            velocity.linvel = Vec2::ZERO;
            commands.entity(arrow_entity).remove::<Arrow>();
            continue;
        }
        for subject_entity in intersections_with(arrow_entity, &context) {
            let Ok((subject_kingdom, mut health)) = subject_query.get_mut(subject_entity) else {
                continue;
            };
            if !health.is_dead() && subject_kingdom != arrow_kingdom {
                health.damage(1);
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
