use bevy::prelude::*;

use crate::Kingdom;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Frontlines>()
            .add_system(update_frontlines);
    }
}

#[derive(Resource)]
pub struct Frontlines {
    pub elven: Frontline,
    pub monster: Frontline,
}

impl Default for Frontlines {
    fn default() -> Self {
        Self {
            elven: Frontline {
                position: f32::NEG_INFINITY,
                entity: None,
            },
            monster: Frontline {
                position: f32::INFINITY,
                entity: None,
            },
        }
    }
}

pub struct Frontline {
    pub position: f32,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct Health {
    current: u32,
}

impl Health {
    pub fn new(initial: u32) -> Self {
        Self { current: initial }
    }

    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn kill(&mut self) {
        self.current = 0;
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

fn update_frontlines(
    query: Query<(Entity, &Transform, &Kingdom), With<Health>>,
    mut frontlines: ResMut<Frontlines>,
) {
    frontlines.elven.position = f32::NEG_INFINITY;
    frontlines.monster.position = f32::INFINITY;

    frontlines.elven.entity = None;
    frontlines.monster.entity = None;

    for (entity, transform, kingdom) in &query {
        match kingdom {
            Kingdom::Elven => {
                if transform.translation.x > frontlines.elven.position {
                    frontlines.elven.position = transform.translation.x;
                    frontlines.elven.entity = Some(entity);
                }
            }
            Kingdom::Monster => {
                if transform.translation.x < frontlines.monster.position {
                    frontlines.monster.position = transform.translation.x;
                    frontlines.monster.entity = Some(entity);
                }
            }
        }
    }
}
