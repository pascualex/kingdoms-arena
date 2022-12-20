pub enum WeaponsBlueprint {
    Sword,
    Bow(&'static BowBlueprint),
}

pub struct BowBlueprint {
    pub range: f32,
    pub speed: f32,
    pub spread: f32,
    pub recharge_seconds: f32,
}

pub const ELVEN_BOW: BowBlueprint = BowBlueprint {
    range: 15.0,
    speed: 15.0,
    spread: 1.5,
    recharge_seconds: 2.3,
};

pub const ELVEN_FAST_BOW: BowBlueprint = BowBlueprint {
    range: 10.0,
    speed: 25.0,
    spread: 2.0,
    recharge_seconds: 0.3,
};

pub const ELVEN_SNIPER_BOW: BowBlueprint = BowBlueprint {
    range: 100.0,
    speed: 15.0,
    spread: 0.0,
    recharge_seconds: 0.0,
};
