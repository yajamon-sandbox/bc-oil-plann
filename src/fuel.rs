use std::fmt;

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
    HeavyOil,
}
impl fmt::Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fuel::Oil => "Oil",
                Fuel::GaseousFuel => "GaseousFuel",
                Fuel::HeavyOil => "HeavyOil",
            }
        )
    }
}

#[derive(Debug)]
pub struct MJ(pub f64);
impl fmt::Display for MJ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MJ(value) = self;
        write!(f, "{} MJ", value)
    }
}

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
