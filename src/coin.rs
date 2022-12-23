use bevy::prelude::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Coins>().add_system(generate_coins);
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
