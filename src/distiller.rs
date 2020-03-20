use crate::buckets::*;
use crate::fuel::*;

#[derive(Debug)]
pub struct Fluid {
    fuel: Fuel,
    amount: MilliBuckets,
}
macro_rules! fluid {
    ($fuel:expr, $amount:expr) => {
        Fluid {
            fuel: $fuel,
            amount: MilliBuckets($amount),
        }
    };
}
#[derive(Debug)]
pub struct Recipe {
    source: Fluid,
    distillate: Fluid,
    residue: Fluid,
}

pub struct Distiller {}

impl Distiller {
    pub fn recipe(fuel: Fuel) -> Option<Recipe> {
        match fuel {
            Fuel::Oil => Some(Recipe {
                source: fluid!(Fuel::Oil, 8),
                distillate: fluid!(Fuel::GaseousFuel, 16),
                residue: fluid!(Fuel::HeavyOil, 3),
            }),
            _ => None,
        }
    }
}
