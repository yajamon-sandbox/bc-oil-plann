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
pub struct MJ(pub f64);

pub trait Burn {
    fn total(&self) -> MJ;
}

impl Burn for Fuel {
    fn total(&self) -> MJ {
        match self {
            Fuel::Oil => MJ(7500.0),
            Fuel::HeavyOil => MJ(20000.0),
            Fuel::GaseousFuel => MJ(15000.0),
        }
    }
}
