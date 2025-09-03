//! SI prefixes for units. Prefix is just a simple trait, which
//! can be implemented via the `prefix!` macro.

/// Trait for SI prefixes
///
/// Defines the interface for all prefix types created by the [`prefix!`] macro.
/// Prefixes provide a scaling factor and symbol that can be combined with
/// base units to create scaled versions (like kilogram = kilo + gram).
///
/// This trait is automatically implemented by the [`prefix!`] macro and should
/// not be implemented manually in most cases.
///
/// # Examples
///
/// ```
/// use ferrunitas::{prefix, __model::Prefix};
///
/// prefix!(Kilo, 1000, "k");
/// prefix!(Milli, 0.001, "m");
///
/// // Access prefix properties
/// assert_eq!(Kilo::FACTOR, 1000.0);
/// assert_eq!(Kilo::SYMBOL, "k");
/// assert_eq!(Milli::FACTOR, 0.001);
/// assert_eq!(Milli::SYMBOL, "m");
/// ```
pub trait Prefix {
    /// The multiplier for this prefix
    const FACTOR: f64;

    /// The symbol for this prefix
    const SYMBOL: &'static str;
}

/// Implemented by types that can have SI prefixes
///
/// This marker trait indicates that a unit can be combined with prefixes
/// to create scaled versions. It's used in the `unit!` macro when defining
/// base units with the `prefixable` flag.
///
/// Units marked as prefixable can be used with the `unit!(prefix: ...)` syntax
/// to create prefixed versions automatically.
///
/// # Examples
///
/// ```ignore
/// // In unit definitions:
/// unit!(base: Metre, "m", Length; prefixable);  // Metre can have prefixes
/// unit!(prefix: Kilometre, Kilo, Metre);        // Creates prefixed version
///
/// // This works because Metre implements Prefixable
/// let km = Kilometre::new(5.0);  // 5 kilometres
/// let m = Metre::new(5000.0);    // 5000 metres (equivalent)
/// ```
pub trait Prefixable {}

/// Macro to define a prefix with its factor, symbol, and name
///
/// This macro creates SI prefix types that can be used to create prefixed units.
/// A prefix defines a multiplication factor and a symbol that will be combined
/// with base units to create scaled versions (like "kilo" + "metre" = "kilometre").
///
/// Each prefix becomes a zero-sized type that implements the [`Prefix`] trait,
/// storing the conversion factor and symbol as compile-time constants.
///
/// # Arguments
///
/// - `prefix_name`: The identifier for the new prefix type (e.g., `Kilo`, `Milli`)
/// - `factor`: The multiplication factor as a number (e.g., `1000`, `0.001`, `1e-6`)  
/// - `symbol`: The prefix symbol as a string literal (e.g., `"k"`, `"m"`, `"μ"`)
///
/// # Examples
///
/// ## Standard SI Prefixes
///
/// ```
/// use ferrunitas::prefix;
///
/// // Large scale prefixes
/// prefix!(Kilo, 1e3, "k");      // 1,000
/// prefix!(Mega, 1e6, "M");      // 1,000,000
/// prefix!(Giga, 1e9, "G");      // 1,000,000,000
/// prefix!(Tera, 1e12, "T");     // 1,000,000,000,000
///
/// // Small scale prefixes  
/// prefix!(Milli, 1e-3, "m");    // 0.001
/// prefix!(Micro, 1e-6, "μ");    // 0.000001
/// prefix!(Nano, 1e-9, "n");     // 0.000000001
/// prefix!(Pico, 1e-12, "p");    // 0.000000000001
///
/// // Binary prefixes (used in computing)
/// prefix!(Kibi, 1024, "Ki");    // 2^10
/// prefix!(Mebi, 1048576, "Mi"); // 2^20
/// ```
///
/// ## Custom Prefixes
///
/// ```
/// use ferrunitas::prefix;
///
/// // Custom prefixes for specialized applications
/// prefix!(Dozen, 12, "dz");     // 12 items
/// prefix!(Score, 20, "sc");     // 20 items  
/// prefix!(Hundred, 100, "h");   // 100 items
/// prefix!(Gross, 144, "gr");    // 144 items (12 dozen)
///
/// // Fractional custom prefixes
/// prefix!(Half, 0.5, "½");      // Half
/// prefix!(Quarter, 0.25, "¼");  // Quarter
/// ```
///
/// ## Usage with Units
///
/// Once defined, prefixes can be used with the `unit!` macro to create prefixed units:
///
/// ```
/// use ferrunitas::{prefix, unit, quantity, typenum_consts::*};
///
/// // Define a quantity and base unit
/// quantity!(Length: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);
/// unit!(base: Metre, "m", Length; prefixable);
///
/// // Define prefixes
/// prefix!(Kilo, 1000, "k");
/// prefix!(Centi, 0.01, "c");
///
/// // Create prefixed units
/// unit!(prefix: Kilometre, Kilo, Metre);    // Creates Kilometre with symbol "km"
/// unit!(prefix: Centimetre, Centi, Metre);  // Creates Centimetre with symbol "cm"
/// ```
///
/// # Generated Type
///
/// The macro generates a zero-sized struct that implements [`Prefix`]:
///
/// ```ignore
/// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// pub struct YourPrefix;
///
/// impl Prefix for YourPrefix {
///     const FACTOR: f64 = your_factor;
///     const SYMBOL: &'static str = "your_symbol";
/// }
/// ```
#[macro_export]
macro_rules! prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr) => {
        /// Auto-generated prefix type produced by `prefix!` macro.
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $prefix_name;

        impl $crate::__model::Prefix for $prefix_name {
            const FACTOR: f64 = $factor as f64;
            const SYMBOL: &'static str = $symbol;
        }
    };
}
