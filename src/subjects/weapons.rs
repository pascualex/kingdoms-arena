use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::ColliderBundle,
    palette,
    subjects::{ShootingState, Subject},
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(recharge_bows)
            .add_system(shoot_subject_bows)
            .add_system(move_arrows);
    }
}

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

fn recharge_bows(mut query: Query<&mut Bow>, time: Res<Time>) {
    for mut bow in &mut query {
        bow.timer.tick(time.delta());
    }
}

fn shoot_subject_bows(
    mut query: Query<(&Transform, &mut Bow), (With<Subject>, With<ShootingState>)>,
    mut commands: Commands,
) {
    for (transform, mut bow) in &mut query {
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
            Arrow,
        ));
    }
}

fn move_arrows(mut query: Query<&mut Transform, With<Arrow>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x += time.delta_seconds() * 5.0;
    }
}
