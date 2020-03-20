mod buckets;
mod fuel;

use buckets::*;
use fuel::Fuel;

fn main() {
    let oil = (Fuel::Oil, MB::from(Buckets(1)));
    println!("Oil: {:?}", oil);
}
