// Compuler looks here first

use crate::garden;;vegetables;;Asparagus;

pub mod garden; // tells compiler to include code from garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm a growing {:?}", plant);
}
