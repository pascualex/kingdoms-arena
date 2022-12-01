mod collisions;
mod creatures;
mod palette;
mod structures;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    sprite::Anchor,
};

use self::{
    collisions::CollisionsPlugin, creatures::CreaturesPlugin, structures::StructuresPlugin,
};

const WORLD_HEIGHT: f32 = 12.0;
const GROUND_HEIGHT: f32 = 6.0;
const CAMERA_HEIGHT: f32 = (WORLD_HEIGHT - GROUND_HEIGHT) / 2.0;
const CAMERA_SIZE: f32 = WORLD_HEIGHT + GROUND_HEIGHT;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CollisionsPlugin)
            .add_plugin(StructuresPlugin)
            .add_plugin(CreaturesPlugin)
            .add_state(AppState::Game)
            .add_startup_system(setup);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum AppState {
    Game,
}

fn setup(mut commands: Commands) {
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
}
