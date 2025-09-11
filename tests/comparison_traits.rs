//! Tests for comparison traits (PartialEq, PartialOrd) on measures and quantities.

// use ferrunitas::model::quantity::QuantityMarker;
use ferrunitas::Unit;
use ferrunitas::system::*;

#[cfg(test)]
mod comparison_traits_tests {
    use super::*;

    // ===========================
    // MEASURE PARTIAL_EQ TESTS
    // ===========================

    #[test]
    fn test_measure_eq_same_measure_same_value() {
        let mass1 = Kilogram::new(5.0);
        let mass2 = Kilogram::new(5.0);
        assert_eq!(mass1, mass2);
    }

    #[test]
    fn test_measure_eq_same_measure_different_value() {
        let mass1 = Kilogram::new(5.0);
        let mass2 = Kilogram::new(3.0);
        assert_ne!(mass1, mass2);
    }

    #[test]
    fn test_measure_eq_same_quantity_different_measures_equal_when_converted() {
        let mass1 = Kilogram::new(2.0);
        let mass2 = Gram::new(2000.0); // 2 kg in grams
        assert_eq!(mass1.into_q(), mass2.into_q());
        assert!(mass1.is_equal_to(&mass2))
    }

    #[test]
    fn test_measure_eq_same_quantity_different_measures_not_equal_when_different() {
        let mass1 = Kilogram::new(2.0);
        let mass2 = Gram::new(1500.0); // 1.5 kg in grams
        assert_ne!(mass1.into_q(), mass2.into_q()); // Should not be equal
    }

    /// ```compile_fail
    /// use crate::measures::*;
    ///
    /// fn bad_eq() {
    ///     let mass = Kilogram::new(5.0);
    ///     let time = Second::new(5.0);
    ///     assert_eq!(mass, time); // should NOT compile
    /// }
    /// ```

    // ===========================
    // QUANTITY PARTIAL_EQ TESTS
    // ===========================

    #[test]
    fn test_quantity_eq_same_value() {
        let mass1: Mass = Kilogram::new(5.0).into_q();
        let mass2: Mass = Kilogram::new(5.0).into_q();
        assert_eq!(mass1, mass2);
    }

    #[test]
    fn test_quantity_eq_different_value() {
        let mass1: Mass = Kilogram::new(5.0).into_q();
        let mass2: Mass = Kilogram::new(3.0).into_q();
        assert_ne!(mass1, mass2);
    }

    #[test]
    fn test_quantity_eq_different_quantity_types() {
        let _mass: Mass = Kilogram::new(5.0).into_q();
        let _length: Length = Metre::new(5.0).into_q();

        assert_ne!(
            core::any::type_name::<Mass>(),
            core::any::type_name::<Length>()
        );
    }

    // ===========================
    // MEASURE PARTIAL_ORD TESTS
    // ===========================

    #[test]
    fn test_measure_ord_same_measure_less_than() {
        let length1 = Kilometre::new(3.0);
        let length2 = Kilometre::new(5.0);
        assert!(length1 < length2);
        assert!(length1 <= length2);
    }

    #[test]
    fn test_measure_ord_same_measure_greater_than() {
        let length1 = Kilometre::new(7.0);
        let length2 = Kilometre::new(5.0);
        assert!(length1 > length2);
        assert!(length1 >= length2);
    }

    #[test]
    fn test_measure_ord_same_measure_equal() {
        let length1 = Kilometre::new(5.0);
        let length2 = Kilometre::new(5.0);
        assert!(length1 <= length2);
        assert!(length1 >= length2);
        assert_eq!(
            length1.partial_cmp(&length2),
            Some(core::cmp::Ordering::Equal)
        );
    }

    #[test]
    fn test_measure_ord_different_measures_same_quantity() {
        let length1 = Kilometre::new(2.0);
        let length2 = Metre::new(2500.0); // 2.5 km
        assert!(length1.into_q() < length2.into_q()); // 2 km < 2.5 km
        assert!(length2.into_q() > length1.into_q()); // 2.5 km > 2 km
    }

    /// ```compile_fail
    /// use crate::measures::*;
    ///
    /// fn bad_eq() {
    ///     let length1 = Kilometre::new(5.0);
    ///     let length2 = Metre::new(5.0);
    ///     assert!(length1 < length2); // should NOT compile
    /// }
    /// ```

    #[test]
    fn test_measure_ord_different_measures_equal_when_converted() {
        let length1 = Kilometre::new(3.0);
        let length2 = Metre::new(3000.0); // 3 km
        let q1 = length1.into_q();
        let q2 = length2.into_q();
        assert!(q1 <= q2);
        assert!(q1 >= q2);
        assert_eq!(q1.partial_cmp(&q2), Some(core::cmp::Ordering::Equal));
    }

    // ===========================
    // QUANTITY PARTIAL_ORD TESTS
    // ===========================

    #[test]
    fn test_quantity_ord_less_than() {
        let length1: Length = Kilometre::new(3.0).into_q();
        let length2: Length = Kilometre::new(5.0).into_q();
        assert!(length1 < length2);
        assert!(length1 <= length2);
    }

    #[test]
    fn test_quantity_ord_greater_than() {
        let length1: Length = Kilometre::new(7.0).into_q();
        let length2: Length = Kilometre::new(5.0).into_q();
        assert!(length1 > length2);
        assert!(length1 >= length2);
    }

    #[test]
    fn test_quantity_ord_equal() {
        let length1: Length = Kilometre::new(5.0).into_q();
        let length2: Length = Kilometre::new(5.0).into_q();
        assert!(length1 <= length2);
        assert!(length1 >= length2);
        assert_eq!(
            length1.partial_cmp(&length2),
            Some(core::cmp::Ordering::Equal)
        );
    }

    #[test]
    fn test_quantity_ord_from_different_measures() {
        let length1: Length = Kilometre::new(2.0).into_q();
        let length2: Length = Metre::new(2500.0).into_q(); // 2.5 km
        assert!(length1 < length2);
        assert!(length2 > length1);
    }

    #[test]
    fn test_quantity_ord_from_different_measures_equal_values() {
        let length1: Length = Kilometre::new(3.0).into_q();
        let length2: Length = Metre::new(3000.0).into_q(); // 3 km
        assert!(length1 <= length2);
        assert!(length1 >= length2);
        assert_eq!(
            length1.partial_cmp(&length2),
            Some(core::cmp::Ordering::Equal)
        );
    }

    #[test]
    fn test_quantity_ord_precision() {
        let energy1: Energy = Joule::new(10.0).into_q();
        let energy2: Energy = Joule::new(10.000001).into_q();
        assert!(energy1 < energy2);
        assert!(energy2 > energy1);
    }

    // ===========================
    // MIXED MEASURE/QUANTITY COMPARISON TESTS
    // ===========================

    #[test]
    fn test_measure_vs_quantity_eq_same_value() {
        let mass_measure = Kilogram::new(5.0);
        let mass_quantity: Mass = Kilogram::new(5.0).into_q();

        // Units and quantities should be comparable when converted
        assert_eq!(mass_measure.into_q(), mass_quantity);
        assert_eq!(mass_quantity.as_measure::<Kilogram>(), mass_measure);
    }

    #[test]
    fn test_measure_vs_quantity_eq_different_value() {
        let mass_measure = Kilogram::new(5.0);
        let mass_quantity: Mass = Kilogram::new(3.0).into_q();

        assert_ne!(mass_measure.into_q(), mass_quantity);
        assert_ne!(mass_quantity.as_measure::<Kilogram>(), mass_measure);
    }

    #[test]
    fn test_measure_vs_quantity_ord() {
        let mass_measure = Kilogram::new(3.0);
        let mass_quantity: Mass = Kilogram::new(5.0).into_q();

        assert!(mass_measure.into_q() < mass_quantity);
        assert!(mass_measure < mass_quantity.as_measure::<Kilogram>());
    }

    #[test]
    fn test_measure_vs_quantity_ord_with_measure_conversion() {
        let mass_measure = Kilogram::new(2.0);
        let mass_quantity: Mass = Gram::new(2500.0).into_q(); // 2.5 kg

        assert!(mass_measure.into_q() < mass_quantity);
        assert!(mass_measure < mass_quantity.as_measure::<Kilogram>());
    }

    // ===========================
    // EDGE CASES
    // ===========================

    #[test]
    fn test_zero_values() {
        let mass1 = Kilogram::new(0.0);
        let mass2 = Gram::new(0.0);
        assert_eq!(mass1.into_q(), mass2.into_q());

        let mass_q1: Mass = Kilogram::new(0.0).into_q();
        let mass_q2: Mass = Kilogram::new(0.0).into_q();
        assert_eq!(mass_q1, mass_q2);
    }

    #[test]
    fn test_negative_values() {
        let temp1 = Kelvin::new(-5.0);
        let temp2 = Kelvin::new(-10.0);
        assert!(temp1 > temp2); // -5 > -10

        let temp_q1: Temperature = Kelvin::new(-5.0).into_q();
        let temp_q2: Temperature = Kelvin::new(-10.0).into_q();
        assert!(temp_q1 > temp_q2);
    }

    #[test]
    fn test_very_small_differences() {
        let mass1 = Kilogram::new(1.0000001);
        let mass2 = Kilogram::new(1.0000000);
        assert!(mass1 > mass2);

        let mass_q1: Mass = Kilogram::new(1.0000001).into_q();
        let mass_q2: Mass = Kilogram::new(1.0000000).into_q();
        assert!(mass_q1 > mass_q2);
    }

    #[test]
    fn test_large_values() {
        let mass1 = Kilogram::new(1e15);
        let mass2 = Kilogram::new(2e15);
        assert!(mass1 < mass2);

        let mass_q1: Mass = Kilogram::new(1e15).into_q();
        let mass_q2: Mass = Kilogram::new(2e15).into_q();
        assert!(mass_q1 < mass_q2);
    }
}
