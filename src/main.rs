mod fuel;
mod buckets;

use fuel::Fuel;
use buckets::*;

fn main() {
    let oil = (Fuel::Oil, MB::from(Buckets(1)));
    println!("Oil: {:?}", oil);
}
