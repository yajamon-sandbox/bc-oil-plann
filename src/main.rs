mod buckets;
mod distiller;
mod fuel;

use buckets::*;
use distiller::*;
use fuel::*;

fn main() {
    analyze(Fluid {
        fuel: Fuel::Oil,
        amount: Buckets(1).into(),
    });
}

fn analyze(fluid: Fluid) {
    let mut plan_number = 0;
    let fluid = &fluid;
    plan_number += 1;
    println!("Plan {}:", plan_number);
    println!(
        "fuel: {}, amount: {}, total: {}",
        fluid.fuel,
        fluid.amount,
        fluid.total()
    );
}
