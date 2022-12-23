use bevy::prelude::*;

use crate::{
    animation::Animation,
    subject::SubjectAnimations,
    weapon::content::{WeaponBlueprint, ELVEN_BOW, ELVEN_FAST_BOW, ELVEN_SNIPER_BOW, GOBLIN_SWORD},
};

pub struct SubjectBlueprint {
    pub name: &'static str,
    pub value: u32,
    pub size: Vec2,
    pub health: u32,
    pub speed: f32,
    pub weapon: &'static WeaponBlueprint,
    pub animations: SubjectAnimations,
}

pub const ELVEN_ARCHER: SubjectBlueprint = SubjectBlueprint {
    name: "Elven archer",
    value: 3,
    size: Vec2::new(1.0, 1.625),
    health: 1,
    speed: 1.5,
    weapon: &ELVEN_BOW,
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.3),
        shooting: Animation::new(14, 7, 0.1),
    },
};

pub const ELVEN_FAST_ARCHER: SubjectBlueprint = SubjectBlueprint {
    name: "Elven fast archer",
    value: 10,
    size: Vec2::new(1.0, 1.625),
    health: 1,
    speed: 3.5,
    weapon: &ELVEN_FAST_BOW,
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.3),
        moving: Animation::new(7, 4, 0.2),
        shooting: Animation::new(14, 7, 0.05),
    },
};

pub const ELVEN_SNIPER_ARCHER: SubjectBlueprint = SubjectBlueprint {
    name: "Elven sniper archer",
    value: 3,
    size: Vec2::new(1.0, 1.625),
    health: 1,
    speed: 1.0,
    weapon: &ELVEN_SNIPER_BOW,
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.45),
        shooting: Animation::new(14, 7, 0.5),
    },
};

pub const GOBLIN_WARRIOR: SubjectBlueprint = SubjectBlueprint {
    name: "Goblin warrior",
    value: 1,
    size: Vec2::new(1.0, 1.375),
    health: 2,
    speed: 2.5,
    weapon: &GOBLIN_SWORD,
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.2),
        shooting: Animation::new(0, 1, 1.0),
    },
};
