mod buckets;
mod distiller;
mod fuel;

use buckets::*;
use distiller::*;
use fuel::Fuel;

fn main() {
    let oil = (Fuel::Oil, MB::from(Buckets(1)));
    println!("Oil: {:?}", oil);
    let recipe = Distiller::recipe(oil.0);
    println!("recipe: {:?}", recipe);
}
