use bevy::prelude::*;

use crate::{structure::NexusSpawnEvent, subject::content::SubjectBlueprint, AppState, Kingdom};

const COINS_PER_SECOND: f32 = 1.5;

pub struct RecruitmentPlugin;

impl Plugin for RecruitmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Coins>()
            .add_event::<RecruitmentEvent>()
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(reset_coins))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(generate_coins))
            .add_system(nexus_spawn_on_recruitment_event);
    }
}

#[derive(Resource, Default)]
pub struct Coins {
    elven: f32,
    monster: f32,
}

impl Coins {
    pub fn get(&self, kingdom: Kingdom) -> f32 {
        match kingdom {
            Kingdom::Elven => self.elven,
            Kingdom::Monster => self.monster,
        }
    }

    pub fn set(&mut self, coins: f32, kingdom: Kingdom) {
        match kingdom {
            Kingdom::Elven => self.elven = coins,
            Kingdom::Monster => self.monster = coins,
        }
    }
}

pub struct RecruitmentEvent {
    pub blueprint: &'static SubjectBlueprint,
    pub kingdom: Kingdom,
}

impl RecruitmentEvent {
    pub fn new(blueprint: &'static SubjectBlueprint, kingdom: Kingdom) -> Self {
        Self { blueprint, kingdom }
    }
}

fn generate_coins(mut coins: ResMut<Coins>, time: Res<Time>) {
    coins.elven += COINS_PER_SECOND * time.delta_seconds();
    coins.monster += COINS_PER_SECOND * time.delta_seconds();
}

fn reset_coins(mut coins: ResMut<Coins>) {
    coins.elven = 0.0;
    coins.monster = 0.0;
}

fn nexus_spawn_on_recruitment_event(
    mut recruitment_events: EventReader<RecruitmentEvent>,
    mut nexus_spawn_events: EventWriter<NexusSpawnEvent>,
    mut coins: ResMut<Coins>,
) {
    for recruitment_event in recruitment_events.iter() {
        let kingdom_coins = coins.get(recruitment_event.kingdom) as u32;
        if recruitment_event.blueprint.value > kingdom_coins {
            continue;
        }
        coins.set(
            (kingdom_coins - recruitment_event.blueprint.value) as f32,
            recruitment_event.kingdom,
        );
        nexus_spawn_events.send(NexusSpawnEvent::new(
            recruitment_event.blueprint,
            recruitment_event.kingdom,
        ));
    }
}
