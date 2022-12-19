use bevy::prelude::*;

use crate::{
    animation::Animation,
    subjects::SubjectAnimations,
    weapons::content::{WeaponsBlueprint, ELVEN_BOW},
};

pub struct SubjectBlueprint {
    pub name: &'static str,
    pub size: Vec2,
    pub speed: f32,
    pub weapon: WeaponsBlueprint,
    pub animations: SubjectAnimations,
}

pub const ELVEN_ARCHER: SubjectBlueprint = SubjectBlueprint {
    name: "Elven archer",
    size: Vec2::new(1.0, 1.625),
    speed: 1.5,
    weapon: WeaponsBlueprint::Bow(ELVEN_BOW),
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.3),
        shooting: Animation::new(14, 7, 0.1),
    },
};

pub const GOBLIN_WARRIOR: SubjectBlueprint = SubjectBlueprint {
    name: "Goblin warrior",
    size: Vec2::new(1.0, 1.4),
    speed: 2.5,
    weapon: WeaponsBlueprint::Sword,
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.3),
        shooting: Animation::new(0, 1, 1.0),
    },
};
