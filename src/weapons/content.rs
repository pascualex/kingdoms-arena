pub enum WeaponsBlueprint {
    Sword,
    Bow(BowBlueprint),
}

pub struct BowBlueprint {
    pub range: f32,
    pub speed: f32,
    pub recharge_seconds: f32,
}

pub const ELVEN_BOW: BowBlueprint = BowBlueprint {
    range: 15.0,
    speed: 15.0,
    recharge_seconds: 2.3,
};
