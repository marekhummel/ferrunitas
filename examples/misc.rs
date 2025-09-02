// For dimensional formatting
use ferrunitas::common::{format_dims, format_unit_dims};

// For access to all definitions to units, quantities and prefixes
use ferrunitas::system::*;

// Crate root access to most common struct Measure and trait Unit to make methods available
use ferrunitas::{Measure, Unit};

// Macros for own definitions (typenum consts for quantity macro)
use ferrunitas::typenum_consts::*;
use ferrunitas::{prefix, quantity, unit};

fn main() {
    // If curious, one can print out the dimensions of a quantity / unit
    println!("Force dimensions:           {}", format_dims::<Force>());
    println!("Acceleration dimensions:    {}", format_dims::<Acceleration>());
    println!("Entropy dimensions:         {}", format_dims::<Entropy>());
    println!();
    println!("MetrePerSecond dimensions:  {}", format_unit_dims::<MetrePerSecond>());
    println!("Farad dimensions:           {}", format_unit_dims::<Farad>());
    println!("MolePerLitre dimensions:    {}", format_unit_dims::<MolePerLitre>());

    // Demonstration for crate level imports
    let _x: Measure<Foot> = Foot::new(3.0);
    // TODO: Meant to fail
    // let y = _x.convert::<Newton>();
    // let a = _x.into_q();
    // let b = a.as_measure::<Newton>();

    // Macro usage, see examples/advanced.rs
    quantity!(MyLength: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);
    prefix!(Quarta, 40, "q");
    unit!(base: Ellbow, "m", MyLength; prefixable);
    println!("MyLength dimensions:           {}", format_dims::<MyLength>());
}
