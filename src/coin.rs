use bevy::prelude::*;

use crate::AppState;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Coins>()
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(reset_coins))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(generate_coins));
    }
}

#[derive(Resource, Default)]
pub struct Coins {
    elven: f32,
}

impl Coins {
    pub fn elven(&self) -> u32 {
        self.elven as u32
    }
}

fn generate_coins(mut coins: ResMut<Coins>, time: Res<Time>) {
    coins.elven += time.delta_seconds();
}

fn reset_coins(mut coins: ResMut<Coins>) {
    coins.elven = 0.0;
}
