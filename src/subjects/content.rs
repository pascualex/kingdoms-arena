use bevy::prelude::*;

use crate::{
    animation::Animation,
    subjects::SubjectAnimations,
    weapons::content::{WeaponsBlueprint, ELVEN_BOW, ELVEN_FAST_BOW, ELVEN_SNIPER_BOW},
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
    weapon: WeaponsBlueprint::Bow(&ELVEN_BOW),
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.3),
        shooting: Animation::new(14, 7, 0.1),
    },
};

pub const ELVEN_FAST_ARCHER: SubjectBlueprint = SubjectBlueprint {
    name: "Fast elven archer",
    size: Vec2::new(1.0, 1.625),
    speed: 3.5,
    weapon: WeaponsBlueprint::Bow(&ELVEN_FAST_BOW),
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.3),
        moving: Animation::new(7, 4, 0.2),
        shooting: Animation::new(14, 7, 0.05),
    },
};

pub const ELVEN_SNIPER_ARCHER: SubjectBlueprint = SubjectBlueprint {
    name: "Elven sniper archer",
    size: Vec2::new(1.0, 1.625),
    speed: 1.0,
    weapon: WeaponsBlueprint::Bow(&ELVEN_SNIPER_BOW),
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.45),
        shooting: Animation::new(14, 7, 0.5),
    },
};

pub const GOBLIN_WARRIOR: SubjectBlueprint = SubjectBlueprint {
    name: "Goblin warrior",
    size: Vec2::new(1.0, 1.375),
    speed: 2.5,
    weapon: WeaponsBlueprint::Sword,
    animations: SubjectAnimations {
        idle: Animation::new(0, 2, 0.6),
        moving: Animation::new(7, 4, 0.2),
        shooting: Animation::new(0, 1, 1.0),
    },
};
