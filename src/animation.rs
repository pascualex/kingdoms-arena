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
    sheet_start_index: usize,
    length: usize,
    mode: AnimationMode,
    timer: Timer,
    index: usize,
    reset: bool,
}

impl AnimationPlayer {
    pub fn new(sprite_entity: Entity, animation: &Animation, mode: AnimationMode) -> Self {
        Self {
            sprite_entity,
            sheet_start_index: animation.start_index,
            length: animation.length,
            mode,
            timer: Timer::from_seconds(animation.interval_seconds, TimerMode::Repeating),
            index: 0,
            reset: true,
        }
    }

    pub fn set(&mut self, animation: &Animation, mode: AnimationMode) {
        self.sheet_start_index = animation.start_index;
        self.length = animation.length;
        self.mode = mode;
        self.timer = Timer::from_seconds(animation.interval_seconds, TimerMode::Repeating);
        self.index = 0;
        self.reset = true;
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.mode, AnimationMode::Once) && self.index == (self.length - 1) && !self.reset
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
    Once,
    Repeating,
}

fn tick_animation_players(
    mut player_query: Query<&mut AnimationPlayer>,
    mut sprite_query: Query<&mut TextureAtlasSprite>,
    time: Res<Time>,
) {
    for mut player in &mut player_query {
        player.timer.tick(time.delta());
        let offset = player.timer.times_finished_this_tick() as usize;
        let new_index = match player.mode {
            AnimationMode::Once => usize::min(player.index + offset, player.length - 1),
            AnimationMode::Repeating => (player.index + offset) % player.length,
        };

        if new_index != player.index || player.reset {
            let mut sprite = sprite_query.get_mut(player.sprite_entity).unwrap();
            sprite.index = player.sheet_start_index + new_index;

            player.index = new_index;
            player.reset = false;
        }
    }
}
