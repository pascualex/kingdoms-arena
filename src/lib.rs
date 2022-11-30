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
        Creature::new(true),
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
        Creature::new(false),
    ));
}

#[derive(Component)]
struct Creature {
    is_player: bool,
}

impl Creature {
    pub fn new(is_player: bool) -> Self {
        Self { is_player }
    }
}

fn movement(mut query: Query<(&Creature, &mut Transform)>, time: Res<Time>) {
    for (creature, mut transform) in &mut query {
        let delta_translation = time.delta_seconds() * 1.0;
        transform.translation.x += match creature.is_player {
            true => delta_translation,
            false => -delta_translation,
        };
    }
}
