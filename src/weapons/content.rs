pub enum WeaponsBlueprint {
    Sword,
    Bow(BowBlueprint),
}

pub struct BowBlueprint {
    pub range: f32,
    pub fire_rate: f32,
}

pub const ELVEN_BOW: BowBlueprint = BowBlueprint {
    range: 10.0,
    fire_rate: 0.3,
};
