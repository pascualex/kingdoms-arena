mod palette;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    sprite::Anchor,
};

const WORLD_HEIGHT: f32 = 12.0;
const GROUND_HEIGHT: f32 = 6.0;
const CAMERA_HEIGHT: f32 = (WORLD_HEIGHT - GROUND_HEIGHT) / 2.0;
const CAMERA_SIZE: f32 = WORLD_HEIGHT + GROUND_HEIGHT;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Game)
            .add_startup_system(setup)
            .add_system(movement);
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
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: palette::DARK_GREEN,
            custom_size: Some(Vec2::new(100.0, GROUND_HEIGHT)),
            anchor: Anchor::TopCenter,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // human
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: palette::LIGHT_PINK,
                custom_size: Some(Vec2::new(0.7, 1.8)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(-10.0, 0.0, 0.0),
            ..default()
        },
        Speed::new(1.0),
        Behaviour::MoveRight,
    ));
    // monster
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: palette::DARK_BLACK,
                custom_size: Some(Vec2::new(0.6, 0.8)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(10.0, 0.0, 0.0),
            ..default()
        },
        Speed::new(2.0),
        Behaviour::MoveLeft,
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

#[derive(Component)]
enum Behaviour {
    MoveRight,
    MoveLeft,
}

fn movement(mut query: Query<(&mut Transform, &Speed, &Behaviour)>, time: Res<Time>) {
    for (mut transform, speed, behaviour) in &mut query {
        transform.translation.x += match behaviour {
            Behaviour::MoveRight => time.delta_seconds() * speed.value,
            Behaviour::MoveLeft => -time.delta_seconds() * speed.value,
        };
    }
}
