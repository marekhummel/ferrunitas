use ferrunitas::system::*;
use ferrunitas::unit;
use ferrunitas::Unit;

unit!(base: NewGram, "ng", Mass; prefixable, factor = 1.0,);

fn main() {
    let value = NewGram::new(5.0);
    println!("Value: {}", value * 2.0);
    println!("Value: {}", 2.0 * value);
}
