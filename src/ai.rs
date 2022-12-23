use bevy::prelude::*;

use crate::{
    recruitment::{Coins, RecruitmentEvent},
    subject::content::{SubjectBlueprint, GOBLIN_WARRIOR},
    Kingdom,
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AiState::new(&GOBLIN_WARRIOR))
            .add_system(recruit_if_affordable);
    }
}

#[derive(Resource)]
struct AiState {
    blueprint: &'static SubjectBlueprint,
}

impl AiState {
    fn new(blueprint: &'static SubjectBlueprint) -> Self {
        Self { blueprint }
    }
}

fn recruit_if_affordable(
    state: Res<AiState>,
    coins: Res<Coins>,
    mut events: EventWriter<RecruitmentEvent>,
) {
    let mut kingdom_coins = coins.get(Kingdom::Monster);
    while kingdom_coins >= state.blueprint.value {
        events.send(RecruitmentEvent::new(state.blueprint, Kingdom::Monster));
        kingdom_coins -= state.blueprint.value;
    }
}
