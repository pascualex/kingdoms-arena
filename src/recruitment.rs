use bevy::prelude::*;

use crate::{structure::NexusSpawnEvent, subject::content::SubjectBlueprint, AppState, Kingdom};

const COINS_PER_SECOND: f32 = 1.0;

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
    pub fn get(&self, kingdom: Kingdom) -> u32 {
        match kingdom {
            Kingdom::Elven => self.elven as u32,
            Kingdom::Monster => self.monster as u32,
        }
    }

    pub fn set(&mut self, coins: u32, kingdom: Kingdom) {
        match kingdom {
            Kingdom::Elven => self.elven = coins as f32,
            Kingdom::Monster => self.monster = coins as f32,
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
}

fn reset_coins(mut coins: ResMut<Coins>) {
    coins.elven = 0.0;
}

fn nexus_spawn_on_recruitment_event(
    mut recruitment_events: EventReader<RecruitmentEvent>,
    mut nexus_spawn_events: EventWriter<NexusSpawnEvent>,
    mut coins: ResMut<Coins>,
) {
    for recruitment_event in recruitment_events.iter() {
        let kingdom_coins = coins.get(recruitment_event.kingdom);
        if recruitment_event.blueprint.value > kingdom_coins {
            continue;
        }
        coins.set(
            kingdom_coins - recruitment_event.blueprint.value,
            recruitment_event.kingdom,
        );
        nexus_spawn_events.send(NexusSpawnEvent::new(
            recruitment_event.blueprint,
            recruitment_event.kingdom,
        ));
    }
}
