mod collisions;
mod palette;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    sprite::Anchor,
};
use bevy_rapier2d::prelude::*;
use collisions::{ColliderBundle, CollisionsPlugin};

const WORLD_HEIGHT: f32 = 12.0;
const GROUND_HEIGHT: f32 = 6.0;
const CAMERA_HEIGHT: f32 = (WORLD_HEIGHT - GROUND_HEIGHT) / 2.0;
const CAMERA_SIZE: f32 = WORLD_HEIGHT + GROUND_HEIGHT;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CollisionsPlugin)
            .add_state(AppState::Game)
            .add_startup_system(setup)
            .add_system(movement)
            .add_system(spawner);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum AppState {
    Game,
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(palette::LIGHT_CYAN),
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(CAMERA_SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0.0, CAMERA_HEIGHT, 0.0),
        ..default()
    });
    // ground
    commands.spawn((
        Name::new("Ground"),
        SpriteBundle {
            sprite: Sprite {
                color: palette::DARK_GREEN,
                custom_size: Some(Vec2::new(100.0, GROUND_HEIGHT)),
                anchor: Anchor::TopCenter,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
    // human
    commands.spawn((
        Name::new("Human spawner"),
        TransformBundle::from_transform(Transform::from_xyz(-10.0, 0.0, 0.0)),
        Spawner::new(
            "Human",
            palette::LIGHT_PINK,
            Vec2::new(0.7, 1.8),
            1.0,
            Behaviour::MoveRight,
            2.0,
        ),
    ));
    // monster
    commands.spawn((
        Name::new("Monster spawner"),
        TransformBundle::from_transform(Transform::from_xyz(10.0, 0.0, 0.0)),
        Spawner::new(
            "Monster",
            palette::DARK_BLACK,
            Vec2::new(0.6, 0.8),
            2.0,
            Behaviour::MoveLeft,
            2.0,
        ),
    ));
}

#[derive(Component)]
struct Speed {
    value: f32,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component, Clone)]
enum Behaviour {
    MoveRight,
    MoveLeft,
}

#[derive(Component)]
struct Spawner {
    name: String,
    color: Color,
    size: Vec2,
    speed: f32,
    behaviour: Behaviour,
    timer: Timer,
    spawn_count: u32,
}

impl Spawner {
    fn new(
        name: impl Into<String>,
        color: Color,
        size: Vec2,
        speed: f32,
        behaviour: Behaviour,
        interval_seconds: f32,
    ) -> Self {
        Self {
            name: name.into(),
            size,
            color,
            speed,
            behaviour,
            timer: Timer::from_seconds(interval_seconds, TimerMode::Repeating),
            spawn_count: 0,
        }
    }
}

fn movement(mut query: Query<(&mut Transform, &Speed, &Behaviour)>, time: Res<Time>) {
    for (mut transform, speed, behaviour) in &mut query {
        transform.translation.x += match behaviour {
            Behaviour::MoveRight => time.delta_seconds() * speed.value,
            Behaviour::MoveLeft => -time.delta_seconds() * speed.value,
        };
    }
}

fn spawner(mut query: Query<(&Transform, &mut Spawner)>, time: Res<Time>, mut commands: Commands) {
    for (transform, mut spawner) in &mut query {
        spawner.timer.tick(time.delta());
        for _ in 0..spawner.timer.times_finished_this_tick() {
            spawner.spawn_count += 1;
            commands.spawn((
                Name::new(format!("{}{}", spawner.name, spawner.spawn_count)),
                SpriteBundle {
                    sprite: Sprite {
                        color: spawner.color,
                        custom_size: Some(spawner.size),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(0.0, spawner.size.y / 2.0, 0.0),
                    ),
                    ..default()
                },
                Speed::new(spawner.speed),
                spawner.behaviour.clone(),
                ColliderBundle::kinematic(Collider::cuboid(
                    spawner.size.x / 2.0,
                    spawner.size.y / 2.0,
                )),
            ));
        }
    }
}
