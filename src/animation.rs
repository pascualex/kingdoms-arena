use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick_animation_players);
    }
}

#[derive(Component)]
pub struct AnimationPlayer {
    sprite_entity: Entity,
    start_index: usize,
    length: usize,
    timer: Timer,
    mode: AnimationMode,
    reset: bool,
}

impl AnimationPlayer {
    pub fn new(sprite_entity: Entity, animation: &Animation, mode: AnimationMode) -> Self {
        Self {
            sprite_entity,
            start_index: animation.start_index,
            length: animation.length,
            timer: Timer::from_seconds(animation.interval_seconds, TimerMode::Repeating),
            mode,
            reset: true,
        }
    }

    pub fn set(&mut self, animation: &Animation, mode: AnimationMode) {
        self.start_index = animation.start_index;
        self.length = animation.length;
        self.timer = Timer::from_seconds(animation.interval_seconds, TimerMode::Repeating);
        self.mode = mode;
        self.reset = true;
    }
}

#[derive(Clone)]
pub struct Animation {
    pub start_index: usize,
    length: usize,
    interval_seconds: f32,
}

impl Animation {
    pub const fn new(start_index: usize, length: usize, interval_seconds: f32) -> Self {
        Self {
            start_index,
            length,
            interval_seconds,
        }
    }
}

#[derive(Clone)]
pub enum AnimationMode {
    Repeating,
}

fn tick_animation_players(
    mut player_query: Query<&mut AnimationPlayer>,
    mut sprite_query: Query<&mut TextureAtlasSprite>,
    time: Res<Time>,
) {
    for mut player in &mut player_query {
        player.timer.tick(time.delta());

        if !player.reset && !player.timer.just_finished() {
            continue;
        }

        let mut sprite = sprite_query.get_mut(player.sprite_entity).unwrap();

        if player.reset {
            sprite.index = player.start_index;
            player.reset = false;
        }

        let index = sprite.index - player.start_index;
        let offset = player.timer.times_finished_this_tick() as usize;
        let new_index = match player.mode {
            // AnimationMode::Once => usize::min(index + offset, player.length - 1),
            AnimationMode::Repeating => (index + offset) % player.length,
        };
        sprite.index = player.start_index + new_index;
    }
}
