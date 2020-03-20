#[derive(Debug)]
pub enum HeatTier {
    Cool,
    Hot,
    Searing,
}

#[derive(Debug)]
pub enum Fuel {
    Oil,
    GaseousFuel,
    HeavyOil
}

#[derive(Debug)]
pub struct MJ(pub i64);

pub trait Burn {
    fn total(&self) -> MJ;
}

impl Burn for Fuel {
    fn total(&self) -> MJ {
        match self {
            Fuel::Oil => MJ(7500),
            Fuel::HeavyOil => MJ(20000),
            Fuel::GaseousFuel => MJ(15000),
        }
    }
}
