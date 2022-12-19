#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod animation;
mod collision;
mod palette;
mod structures;
mod subjects;
mod weapons;

use bevy::{
    asset::Asset, core_pipeline::clear_color::ClearColorConfig, prelude::*,
    render::camera::ScalingMode, sprite::Anchor,
};

use self::{
    animation::AnimationPlugin, structures::StructurePlugin, subjects::SubjectPlugin,
    weapons::WeaponPlugin,
};

// Perfect pixel art: 360.0 / 22.5 = 16.0
const SKY_HEIGHT: f32 = 15.0;
const GROUND_HEIGHT: f32 = 7.5;
const WORLD_EXTENSION: f32 = 20.0;
const GRAVITY_ACCELERATION: f32 = 9.8;
const CAMERA_HEIGHT: f32 = (SKY_HEIGHT - GROUND_HEIGHT) / 2.0;
const CAMERA_SIZE: f32 = SKY_HEIGHT + GROUND_HEIGHT;
const PX_PER_METER: f32 = 8.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationPlugin)
            .add_plugin(StructurePlugin)
            .add_plugin(SubjectPlugin)
            .add_plugin(WeaponPlugin)
            .add_state(AppState::Game)
            .add_startup_system(setup);
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
        transform: Transform::from_xyz(0.0, CAMERA_HEIGHT, 99.9),
        ..default()
    });
    // background
    commands.spawn((
        Name::new("Ground"),
        SpriteBundle {
            sprite: Sprite {
                color: palette::DARK_GREEN,
                custom_size: Some(Vec2::new(WORLD_EXTENSION * 2.0, GROUND_HEIGHT)),
                anchor: Anchor::TopCenter,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Kingdom {
    Elven,
    Monster,
}

pub struct KingdomHandle<T: Asset> {
    pub elven: Handle<T>,
    pub monster: Handle<T>,
}

impl<T: Asset> KingdomHandle<T> {
    pub fn get(&self, kingdom: Kingdom) -> Handle<T> {
        match kingdom {
            Kingdom::Elven => self.elven.clone(),
            Kingdom::Monster => self.monster.clone(),
        }
    }
}
