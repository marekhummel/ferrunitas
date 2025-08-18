# Prefix System Architecture

## Overview
The Ferrunitas unit system has been upgraded to use a **prefix-based architecture** instead of individual unit definitions. This provides better scalability, consistency, and maintainability.

## Key Improvements

### Before (Individual Units)
```rust
// Each prefixed unit was a separate struct with hardcoded factors
derived_unit!(Kilogram, Mass, 1000.0); // Hardcoded factor
derived_unit!(Kilometer, Length, 1000.0); // Duplicate factor
derived_unit!(Centimeter, Length, 0.01); // Different hardcoded factor
derived_unit!(Millimeter, Length, 0.001); // Another hardcoded factor
```

### After (Prefix System)
```rust
// Prefixes are defined once with their factors
pub struct Kilo;
impl Prefix for Kilo {
    fn factor() -> f64 { 1000.0 }
    fn symbol() -> &'static str { "k" }
    fn name() -> &'static str { "kilo" }
}

// Units use prefixes - no hardcoded factors!
prefixed_unit_type!(Kilogram, Kilo, Gram);
prefixed_unit_type!(Kilometer, Kilo, Meter);
```

## Architecture Components

### 1. Prefix Trait
```rust
pub trait Prefix {
    fn factor() -> f64;        // 1000.0 for Kilo
    fn symbol() -> &'static str; // "k" for Kilo
    fn name() -> &'static str;   // "kilo" for Kilo
}
```

### 2. Generic PrefixedUnit
```rust
pub struct PrefixedUnit<P: Prefix, U>(pub f64, PhantomData<P>, PhantomData<U>);
```
- `P`: The prefix type (Kilo, Centi, etc.)
- `U`: The base unit type (Gram, Meter, etc.)
- `f64`: The value in the prefixed unit

### 3. Type Aliases
```rust
pub type Kilogram = PrefixedUnit<Kilo, Gram>;
pub type Kilometer = PrefixedUnit<Kilo, Meter>;
pub type Centimeter = PrefixedUnit<Centi, Meter>;
```

## Available Prefixes

| Prefix | Factor | Symbol | Example |
|--------|--------|--------|---------|
| Kilo   | 1000   | k      | Kilogram, Kilometer |
| Centi  | 0.01   | c      | Centimeter |
| Milli  | 0.001  | m      | Millimeter |
| Micro  | 0.000001 | μ    | Micrometer |
| Mega   | 1,000,000 | M   | Megagram |
| Giga   | 1,000,000,000 | G | Gigameter |

## Usage Examples

### Same API as Before
```rust
let mass: Mass = Kilogram(5.0).into();     // Forward conversion
let kg = Kilogram::from_quantity(mass);    // Backward conversion
```

### Prefix Information Access
```rust
println!("Factor: {}", Kilo::factor());    // 1000
println!("Symbol: {}", Kilo::symbol());    // "k"
println!("Name: {}", Kilo::name());        // "kilo"
```

### Easy Extension
Adding new prefixed units is now just 2 lines:
```rust
prefixed_unit_type!(Micrometer, Micro, Meter);
impl_prefixed_unit_conversions!(Micrometer, Micro, Meter, Length);
```

## Benefits

### 1. **Consistency**
- All kilo-units use the same 1000.0 factor
- No risk of typos in conversion factors
- Systematic approach to unit definitions

### 2. **Scalability**
- Adding new prefixed units requires minimal code
- Prefixes can be reused across different base units
- Easy to add new prefixes system-wide

### 3. **Maintainability**
- Single source of truth for each prefix factor
- Changes to prefix definitions automatically propagate
- Clear separation between prefixes and base units

### 4. **Type Safety**
- Compile-time verification of prefix-unit combinations
- Generic `PrefixedUnit<P, U>` prevents mixing incompatible types
- Same dimensional analysis guarantees

### 5. **Performance**
- Zero runtime overhead - all resolved at compile time
- Same efficient conversions as before
- Perfect precision in round-trip conversions

## Implementation Details

### Macro System
```rust
// Creates the type alias
prefixed_unit_type!(Kilogram, Kilo, Gram);

// Implements conversions and constructor functions
impl_prefixed_unit_conversions!(Kilogram, Kilo, Gram, Mass);
```

### Conversion Logic
```rust
// Forward: PrefixedUnit -> Quantity
unit.value * Prefix::factor()

// Backward: Quantity -> PrefixedUnit
quantity.value() / Prefix::factor()
```

## Migration Impact

### ✅ **No Breaking Changes**
- All existing APIs work exactly the same
- `Kilogram(5.0).into()` still works
- `Kilogram::from_quantity()` still works
- All unit operations and physics calculations unchanged

### ✅ **Internal Architecture Improved**
- Kilogram is now `PrefixedUnit<Kilo, Gram>` instead of separate struct
- Kilometer is now `PrefixedUnit<Kilo, Meter>` instead of separate struct
- Consistent prefix factors automatically applied

## Future Extensions

### Easy to Add
- **New Prefixes**: Nano, Pico, Tera, etc.
- **New Base Units**: Ampere, Kelvin, Candela, etc.
- **New Prefixed Units**: Any combination of existing prefixes + base units

### Potential Enhancements
- Display formatting with prefix symbols (`5 kg` instead of `5000 g`)
- Parse prefixed unit strings (`"5 km"` → `Kilometer(5.0)`)
- Automatic prefix selection for optimal display range
