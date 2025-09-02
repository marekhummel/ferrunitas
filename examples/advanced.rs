use ferrunitas::typenum_consts::*;
use ferrunitas::Unit;
use ferrunitas::{prefix, quantity, unit};

// Demonstration on how to use the macros to define new units / quantities / prefixes.
// Obviously these types are defined in the library already.

// ===== QUANTITIES =====
// New quantities. Defined by either a list of 7 exponents (M, L, T, I, Th, N, J) or
// by compounding existing ones. Any exponents are from typenum, reexported in ferrunitas::typenum_consts
quantity!(Length: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);
quantity!(Time: M Z0, L Z0, T P1, I Z0, Th Z0, N Z0, J Z0);
quantity!(Velocity: [(Length, P1), (Time, N1)]);

// ===== PREFIXES =====
// New prefixes can be set to create prefixed units more easily later
prefix!(Milli, 1e-3, "m");
prefix!(Kilo, 1e3, "k");

// ===== UNITS =====
// Units can be created in a multitude of ways. Note that any unit can be made prefixable, to allow
// the combination with prefixes.

// The simplest one is by defining a base unit for a quantity.
unit!(base: Metre, "m", Length; prefixable);
unit!(base: Second, "s", Time; prefixable);

// For convienience, you can also define prefixed units directly.
unit!(prefix: Kilometre, Kilo, Metre);
unit!(prefix: Millisecond, Milli, Second);

// For units that are multitudes of another, you can derive it
unit!(derived: Foot, "ft", (0.3048, Metre));
unit!(derived: Mile, "mi", (5280, Foot));
unit!(derived: Hour, "h", (3600, Second));

// Lastly, instead of using factors we can combine existing units to create a new one.

// Same as the following, as metre and second are base units: unit!(base: MetrePerSecond, "m/s", Velocity;);
unit!(compound: MetrePerSecond, "m/s", [(Metre, P1), (Second, N1)]);

// Instead of: unit!(derived: KilometrePerHour, "km/h", (0.277778, MetrePerSecond));
unit!(compound: KilometrePerHour, "km/h", [(Kilometre, P1), (Hour, N1)]);

// If a multiple of a unit is not given, you can provide a factor instead:
unit!(compound: YardsPerMinute, "yd/min", [(3, Foot, P1), (60, Second, N1)]);

fn main() {
    // Working with those units works just like the other examples
    let length = Foot::new(500.0);
    let time = Second::new(90.0);

    let speed = length / time;
    println!(
        "{:.2} in {:.2} equal a speed of {:.2} or {:.2}",
        length,
        time,
        speed.as_measure::<KilometrePerHour>(),
        speed.as_measure::<YardsPerMinute>()
    );
}
