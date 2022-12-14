use bevy::prelude::*;

use crate::{animation::Animation, subjects::SubjectAnimations};

pub struct SubjectBlueprint {
    pub name: &'static str,
    pub size: Vec2,
    pub speed: f32,
    pub weapon: WeaponType,
    pub animations: SubjectAnimations,
}

pub enum WeaponType {
    Sword,
    Bow,
}

pub const ARCHER: SubjectBlueprint = SubjectBlueprint {
    name: "Archer",
    size: Vec2::new(1.1, 1.8),
    speed: 1.5,
    weapon: WeaponType::Bow,
    animations: SubjectAnimations {
        moving: Animation::new(9, 4, 0.3),
        shooting: Animation::new(0, 2, 0.6),
    },
};

pub const MONSTER: SubjectBlueprint = SubjectBlueprint {
    name: "Monster",
    size: Vec2::new(1.0, 1.4),
    speed: 2.5,
    weapon: WeaponType::Sword,
    animations: SubjectAnimations {
        moving: Animation::new(9, 4, 0.3),
        shooting: Animation::new(0, 2, 0.6),
    },
};
