use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::{intersections_with, ColliderBundle},
    palette,
    subjects::{despawn_dead_subjects, Health, ShootingState, Subject},
    Kingdom,
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(swing_subject_swords.before(despawn_dead_subjects))
            .add_system(recharge_bows)
            .add_system(shoot_subject_bows.after(recharge_bows))
            .add_system(move_arrows.after(shoot_subject_bows))
            .add_system(
                collide_arrows
                    .after(move_arrows)
                    .after(swing_subject_swords)
                    .before(despawn_dead_subjects),
            )
            .add_system(despawn_arrows.after(shoot_subject_bows));
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
pub struct Arrow {
    timer: Timer,
}

impl Arrow {
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
    mut query: Query<(&Transform, &Kingdom, &mut Bow), (With<Subject>, With<ShootingState>)>,
    mut commands: Commands,
) {
    for (transform, kingdom, mut bow) in &mut query {
        if !bow.timer.finished() {
            continue;
        }
        bow.timer.reset();
        commands.spawn((
            Name::new("Arrow"),
            SpriteBundle {
                sprite: Sprite {
                    color: palette::DARK_YELLOW,
                    custom_size: Some(Vec2::new(0.1, 0.1)),
                    ..default()
                },
                transform: Transform::from_translation(transform.translation),
                ..default()
            },
            ColliderBundle::kinematic(Collider::cuboid(0.05, 0.05)),
            kingdom.clone(),
            Arrow::new(5.0),
        ));
    }
}

fn move_arrows(mut query: Query<&mut Transform, With<Arrow>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x += time.delta_seconds() * 10.0;
    }
}

fn collide_arrows(
    arrow_query: Query<(Entity, &Kingdom), With<Arrow>>,
    mut subject_query: Query<(&Kingdom, &mut Health), With<Subject>>,
    context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (arrow_entity, arrow_kingdom) in &arrow_query {
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

fn despawn_arrows(mut query: Query<(Entity, &mut Arrow)>, time: Res<Time>, mut commands: Commands) {
    for (entity, mut arrow) in &mut query {
        arrow.timer.tick(time.delta());
        if arrow.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
