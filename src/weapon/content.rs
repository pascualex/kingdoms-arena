pub struct WeaponBlueprint {
    pub damage: u32,
    pub kind: WeaponKind,
}

pub enum WeaponKind {
    Sword,
    Bow(BowBlueprint),
}

pub struct BowBlueprint {
    pub range: f32,
    pub speed: f32,
    pub spread: f32,
    pub recharge_seconds: f32,
}

pub const ELVEN_BOW: WeaponBlueprint = WeaponBlueprint {
    damage: 1,
    kind: WeaponKind::Bow(BowBlueprint {
        range: 15.0,
        speed: 15.0,
        spread: 1.5,
        recharge_seconds: 2.3,
    }),
};

pub const ELVEN_FAST_BOW: WeaponBlueprint = WeaponBlueprint {
    damage: 1,
    kind: WeaponKind::Bow(BowBlueprint {
        range: 10.0,
        speed: 25.0,
        spread: 2.0,
        recharge_seconds: 0.3,
    }),
};

pub const ELVEN_SNIPER_BOW: WeaponBlueprint = WeaponBlueprint {
    damage: 2,
    kind: WeaponKind::Bow(BowBlueprint {
        range: 100.0,
        speed: 15.0,
        spread: 0.0,
        recharge_seconds: 0.0,
    }),
};

pub const GOBLIN_SWORD: WeaponBlueprint = WeaponBlueprint {
    damage: 1,
    kind: WeaponKind::Sword,
};
