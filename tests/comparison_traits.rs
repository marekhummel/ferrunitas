// use ferrunitas::model::quantity::QuantityMarker;
use ferrunitas::model::unit::Unit;
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
        let mass1: Mass = Mass::new(5.0);
        let mass2: Mass = Mass::new(5.0);
        assert_eq!(mass1, mass2);
    }

    #[test]
    fn test_quantity_eq_different_value() {
        let mass1: Mass = Mass::new(5.0);
        let mass2: Mass = Mass::new(3.0);
        assert_ne!(mass1, mass2);
    }

    #[test]
    fn test_quantity_eq_different_quantity_types() {
        let _mass: Mass = Mass::new(5.0);
        let _length: Length = Length::new(5.0);

        assert_ne!(
            std::any::type_name::<Mass>(),
            std::any::type_name::<Length>()
        );
    }

    // ===========================
    // MEASURE PARTIAL_ORD TESTS
    // ===========================

    #[test]
    fn test_measure_ord_same_measure_less_than() {
        let mass1 = Kilogram::new(3.0);
        let mass2 = Kilogram::new(5.0);
        assert!(mass1 < mass2);
        assert!(mass1 <= mass2);
    }

    #[test]
    fn test_measure_ord_same_measure_greater_than() {
        let mass1 = Kilogram::new(7.0);
        let mass2 = Kilogram::new(5.0);
        assert!(mass1 > mass2);
        assert!(mass1 >= mass2);
    }

    #[test]
    fn test_measure_ord_same_measure_equal() {
        let mass1 = Kilogram::new(5.0);
        let mass2 = Kilogram::new(5.0);
        assert!(mass1 <= mass2);
        assert!(mass1 >= mass2);
        assert_eq!(mass1.partial_cmp(&mass2), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn test_measure_ord_different_measures_same_quantity() {
        let mass1 = Kilogram::new(2.0);
        let mass2 = Gram::new(2500.0); // 2.5 kg
        assert!(mass1.into_q() < mass2.into_q()); // 2 kg < 2.5 kg
        assert!(mass2.into_q() > mass1.into_q()); // 2.5 kg > 2 kg
    }

    /// ```compile_fail
    /// use crate::measures::*;
    ///
    /// fn bad_eq() {
    ///     let mass1 = Kilogram::new(5.0);
    ///     let mass2 = Gram::new(5.0);
    ///     assert!(mass1 < mass2); // should NOT compile
    /// }
    /// ```

    #[test]
    fn test_measure_ord_different_measures_equal_when_converted() {
        let mass1 = Kilogram::new(3.0);
        let mass2 = Gram::new(3000.0); // 3 kg
        let q1 = mass1.into_q();
        let q2 = mass2.into_q();
        assert!(q1 <= q2);
        assert!(q1 >= q2);
        assert_eq!(q1.partial_cmp(&q2), Some(std::cmp::Ordering::Equal));
    }

    // ===========================
    // QUANTITY PARTIAL_ORD TESTS
    // ===========================

    #[test]
    fn test_quantity_ord_less_than() {
        let mass1: Mass = Mass::new(3.0);
        let mass2: Mass = Mass::new(5.0);
        assert!(mass1 < mass2);
        assert!(mass1 <= mass2);
    }

    #[test]
    fn test_quantity_ord_greater_than() {
        let mass1: Mass = Mass::new(7.0);
        let mass2: Mass = Mass::new(5.0);
        assert!(mass1 > mass2);
        assert!(mass1 >= mass2);
    }

    #[test]
    fn test_quantity_ord_equal() {
        let mass1: Mass = Mass::new(5.0);
        let mass2: Mass = Mass::new(5.0);
        assert!(mass1 <= mass2);
        assert!(mass1 >= mass2);
        assert_eq!(mass1.partial_cmp(&mass2), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn test_quantity_ord_from_different_measures() {
        let mass1: Mass = Kilogram::new(2.0).into_q();
        let mass2: Mass = Gram::new(2500.0).into_q(); // 2.5 kg
        assert!(mass1 < mass2);
        assert!(mass2 > mass1);
    }

    #[test]
    fn test_quantity_ord_from_different_measures_equal_values() {
        let mass1: Mass = Kilogram::new(3.0).into_q();
        let mass2: Mass = Gram::new(3000.0).into_q(); // 3 kg
        assert!(mass1 <= mass2);
        assert!(mass1 >= mass2);
        assert_eq!(mass1.partial_cmp(&mass2), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn test_quantity_ord_precision() {
        let energy1: Energy = Energy::new(10.0);
        let energy2: Energy = Energy::new(10.000001);
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

        let mass_q1: Mass = Mass::new(0.0);
        let mass_q2: Mass = Mass::new(0.0);
        assert_eq!(mass_q1, mass_q2);
    }

    #[test]
    fn test_negative_values() {
        let temp1 = Kelvin::new(-5.0);
        let temp2 = Kelvin::new(-10.0);
        assert!(temp1 > temp2); // -5 > -10

        let temp_q1: Temperature = Temperature::new(-5.0);
        let temp_q2: Temperature = Temperature::new(-10.0);
        assert!(temp_q1 > temp_q2);
    }

    #[test]
    fn test_very_small_differences() {
        let mass1 = Kilogram::new(1.0000001);
        let mass2 = Kilogram::new(1.0000000);
        assert!(mass1 > mass2);

        let mass_q1: Mass = Mass::new(1.0000001);
        let mass_q2: Mass = Mass::new(1.0000000);
        assert!(mass_q1 > mass_q2);
    }

    #[test]
    fn test_large_values() {
        let mass1 = Kilogram::new(1e15);
        let mass2 = Kilogram::new(2e15);
        assert!(mass1 < mass2);

        let mass_q1: Mass = Mass::new(1e15);
        let mass_q2: Mass = Mass::new(2e15);
        assert!(mass_q1 < mass_q2);
    }
}
