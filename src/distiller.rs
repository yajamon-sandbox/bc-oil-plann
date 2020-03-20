use crate::buckets::*;
use crate::fuel::*;

#[derive(Debug)]
pub struct Fluid {
    pub fuel: Fuel,
    pub amount: MilliBuckets,
}
macro_rules! fluid {
    ($fuel:expr, $amount:expr) => {
        Fluid {
            fuel: $fuel,
            amount: MilliBuckets($amount),
        }
    };
}

impl Burn for Fluid {
    fn total(&self) -> MJ {
        let MJ(total) = self.fuel.total();
        let MilliBuckets(amount) = self.amount;
        MJ(total / 1000.0 * amount as f64)
    }
}

#[derive(Debug)]
pub struct Recipe {
    source: Fluid,
    distillate: Fluid,
    residue: Fluid,
    cost: MJ,
}

pub struct Distiller {}

impl Distiller {
    pub fn recipe(fuel: Fuel) -> Option<Recipe> {
        match fuel {
            Fuel::Oil => Some(Recipe {
                source: fluid!(Fuel::Oil, 8),
                distillate: fluid!(Fuel::GaseousFuel, 16),
                residue: fluid!(Fuel::HeavyOil, 3),
                cost: MJ(32.0),
            }),
            _ => None,
        }
    }
}
