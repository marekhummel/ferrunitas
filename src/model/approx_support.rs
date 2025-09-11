use crate::model::{
    dimension::Dimensioned,
    measure::Measure,
    quantity::{Quantity, QuantityTag},
    unit::Unit,
};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

// ====================== Approx impls for Quantity ===================== //

impl<D: Dimensioned, T: QuantityTag> AbsDiffEq for Quantity<D, T> {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_eq(&other.value, epsilon)
    }
}

impl<D: Dimensioned, T: QuantityTag> RelativeEq for Quantity<D, T> {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.value.relative_eq(&other.value, epsilon, max_relative)
    }
}

impl<D: Dimensioned, T: QuantityTag> UlpsEq for Quantity<D, T> {
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }
}

// ====================== Approx impls for Measure ===================== //

impl<U: Unit> AbsDiffEq for Measure<U> {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_eq(&other.value, epsilon)
    }
}

impl<U: Unit> RelativeEq for Measure<U> {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.value.relative_eq(&other.value, epsilon, max_relative)
    }
}

impl<U: Unit> UlpsEq for Measure<U> {
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Unit, system::*};
    use approx::{assert_abs_diff_eq, assert_relative_eq, assert_ulps_eq};

    // ====================== Measure Tests ===================== //

    #[test]
    fn test_measure_abs_diff_eq() {
        let length1 = Metre::new(1.0);
        let length2 = Metre::new(1.0000001);

        // Should be approximately equal with custom epsilon
        assert_abs_diff_eq!(length1, length2, epsilon = 1e-6);

        // Test with different magnitudes
        let mass1 = Kilogram::new(100.0);
        let mass2 = Kilogram::new(100.0000001);
        assert_abs_diff_eq!(mass1, mass2, epsilon = 1e-6);
    }

    #[test]
    fn test_measure_relative_eq() {
        let time1 = Second::new(100.0);
        let time2 = Second::new(100.01); // 0.01% difference

        // Should be approximately equal with relative tolerance
        assert_relative_eq!(time1, time2, max_relative = 1e-3);

        // Test with larger values
        let distance1 = Metre::new(1000000.0);
        let distance2 = Metre::new(1000100.0); // 0.01% difference
        assert_relative_eq!(distance1, distance2, max_relative = 1e-3);
    }

    #[test]
    fn test_measure_ulps_eq() {
        let mass1 = Kilogram::new(273.15);
        let mass2 = Kilogram::new(273.15 + f64::EPSILON * 2.0);

        // Should be approximately equal within 4 ULPs
        assert_ulps_eq!(mass1, mass2, max_ulps = 4);

        // Test with zero values
        let zero1 = Metre::new(0.0);
        let zero2 = Metre::new(0.0);
        assert_ulps_eq!(zero1, zero2);
    }

    // ====================== Quantity Tests ===================== //

    #[test]
    fn test_quantity_abs_diff_eq() {
        let length1 = Length::new(1.0);
        let length2 = Length::new(1.0000001);

        // Should be approximately equal with custom epsilon
        assert_abs_diff_eq!(length1, length2, epsilon = 1e-6);

        // Test with different magnitudes
        let mass1 = Mass::new(50.0);
        let mass2 = Mass::new(50.0000001);
        assert_abs_diff_eq!(mass1, mass2, epsilon = 1e-6);
    }

    #[test]
    fn test_quantity_relative_eq() {
        let time1 = Time::new(100.0);
        let time2 = Time::new(100.05); // 0.05% difference

        // Should be approximately equal with relative tolerance
        assert_relative_eq!(time1, time2, max_relative = 1e-3);

        // Test with very large values
        let mass1 = Mass::new(1e20);
        let mass2 = Mass::new(1e20 + 1e15); // Small relative difference
        assert_relative_eq!(mass1, mass2, max_relative = 1e-4);
    }

    #[test]
    fn test_quantity_ulps_eq() {
        let length1 = Length::new(6.0);
        let length2 = Length::new(6.0 + f64::EPSILON * 3.0);

        // Should be approximately equal within ULP tolerance
        assert_ulps_eq!(length1, length2, max_ulps = 8);

        // Test with negative values
        let time1 = Time::new(-5.0);
        let time2 = Time::new(-5.0000001);
        assert_abs_diff_eq!(time1, time2, epsilon = 1e-6);
    }
}
