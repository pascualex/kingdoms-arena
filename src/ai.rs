use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{structures::NexusSpawnEvent, subjects::content::GOBLIN_WARRIOR, Kingdom};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AiPlayer::new(0.5))
            .add_system(tick_ai_player);
    }
}

#[derive(Resource)]
struct AiPlayer {
    stopwatch: Stopwatch,
    average_interval: Duration,
    next_interval: Duration,
}

impl AiPlayer {
    fn new(interval_seconds: f32) -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            average_interval: Duration::from_secs_f32(interval_seconds),
            next_interval: Duration::ZERO,
        }
    }
}

fn tick_ai_player(
    mut ai_player: ResMut<AiPlayer>,
    mut events: EventWriter<NexusSpawnEvent>,
    time: Res<Time>,
) {
    ai_player.stopwatch.tick(time.delta());
    while ai_player.stopwatch.elapsed() >= ai_player.next_interval {
        let remaining = ai_player.stopwatch.elapsed() - ai_player.next_interval;
        ai_player.stopwatch.set_elapsed(remaining);

        let random_offset = 0.5 + fastrand::f32();
        ai_player.next_interval = ai_player.average_interval.mul_f32(random_offset);

        let event = NexusSpawnEvent::new(GOBLIN_WARRIOR, Kingdom::Monster);
        events.send(event);
    }
}
