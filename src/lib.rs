use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Game).add_startup_system(setup);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum AppState {
    Game,
}

fn setup() {}
