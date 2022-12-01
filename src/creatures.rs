use bevy::prelude::*;

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement);
    }
}

#[derive(Component)]
pub struct Speed {
    value: f32,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component, Clone)]
pub enum Behaviour {
    MoveRight,
    MoveLeft,
}

fn movement(mut query: Query<(&mut Transform, &Speed, &Behaviour)>, time: Res<Time>) {
    for (mut transform, speed, behaviour) in &mut query {
        transform.translation.x += match behaviour {
            Behaviour::MoveRight => time.delta_seconds() * speed.value,
            Behaviour::MoveLeft => -time.delta_seconds() * speed.value,
        };
    }
}
