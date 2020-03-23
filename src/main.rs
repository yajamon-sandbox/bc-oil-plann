mod buckets;
mod distiller;
mod fuel;

use buckets::*;
use distiller::*;
use fuel::*;

fn main() {
    let fluid = Fluid {
        fuel: Fuel::Oil,
        amount: Buckets(1).into(),
    };
    analyze(&fluid);
}

fn analyze(fluid: &Fluid) {
    let fluids = vec![fluid];

    analyze_step(&fluids);
}

fn analyze_step(fluids: &Vec<&Fluid>) {
    let mut total_mj = MJ(0.0);
    for fluid in fluids {
        println!(
            "fuel: {},\tamount: {},\ttotalMJ: {}",
            fluid.fuel,
            fluid.amount,
            fluid.total()
        );
        total_mj = MJ(total_mj.0 + fluid.total().0)
    }
    println!("all fluids totalMJ: {}", total_mj);
}
