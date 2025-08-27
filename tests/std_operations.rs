use ferrunitas::model::quantity::{Quantity, QuantityMarker};
use ferrunitas::model::unit::Unit;
use ferrunitas::system::*;
use typenum::*;

#[cfg(test)]
mod std_operations_tests {
    use super::*;

    // ===========================
    // ADD TESTS
    // ===========================

    #[test]
    fn test_add_same_unit() {
        let mass1 = Kilogram::new(2.0);
        let mass2 = Kilogram::new(3.0);
        let result = mass1 + mass2;
        assert_eq!(result, Kilogram::new(5.0));
    }

    #[test]
    fn test_add_same_quantity_different_units() {
        let mass1 = Kilogram::new(2.0);
        let mass2 = Gram::new(500.0);
        let result = mass1 + mass2;
        assert_eq!(result, Kilogram::new(2.5));
    }

    #[test]
    fn test_add_unit_quantity() {
        let mass_unit = Kilogram::new(3.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        let result = mass_unit + mass_quantity;
        assert_eq!(result, Kilogram::new(5.0));
    }

    #[test]
    fn test_add_quantity_unit() {
        let mass_quantity = Kilogram::new(3.0).into_q();
        let mass_unit = Gram::new(2000.0);
        let result = mass_quantity + mass_unit;
        assert_eq!(result, Mass::new(5.0));
    }

    // ===========================
    // ADD_ASSIGN TESTS
    // ===========================

    #[test]
    fn test_add_assign_same_unit() {
        let mut length = Metre::new(10.0);
        let length2 = Metre::new(5.0);
        length += length2;
        assert_eq!(length, Metre::new(15.0));
    }

    #[test]
    fn test_add_assign_same_quantity_different_units() {
        let mut length = Metre::new(1.0);
        let length2 = Centimetre::new(50.0);
        length += length2;
        assert_eq!(length, Metre::new(1.5));
    }

    #[test]
    fn test_add_assign_unit_quantity() {
        let mut mass = Kilogram::new(3.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        mass += mass_quantity;
        assert_eq!(mass, Kilogram::new(5.0));
    }

    #[test]
    fn test_add_assign_quantity_unit() {
        let mut mass_quantity = Kilogram::new(3.0).into_q();
        let mass_unit = Gram::new(2000.0);
        mass_quantity += mass_unit;
        assert_eq!(mass_quantity, Mass::new(5.0));
    }

    // ===========================
    // SUB TESTS
    // ===========================

    #[test]
    fn test_sub_same_unit() {
        let time1 = Second::new(10.0);
        let time2 = Second::new(3.0);
        let result = time1 - time2;
        assert_eq!(result, Second::new(7.0));
    }

    #[test]
    fn test_sub_same_quantity_different_units() {
        let time1 = Minute::new(2.0);
        let time2 = Second::new(30.0);
        let result = time1 - time2;
        assert_eq!(result, Minute::new(1.5));
    }

    #[test]
    fn test_sub_unit_quantity() {
        let mass_unit = Kilogram::new(5.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        let result = mass_unit - mass_quantity;
        assert_eq!(result, Kilogram::new(3.0));
    }

    #[test]
    fn test_sub_quantity_unit() {
        let mass_quantity = Kilogram::new(5.0).into_q();
        let mass_unit = Gram::new(2000.0);
        let result = mass_quantity - mass_unit;
        assert_eq!(result, Mass::new(3.0));
    }

    // ===========================
    // SUB_ASSIGN TESTS
    // ===========================

    #[test]
    fn test_sub_assign_same_unit() {
        let mut area = SquareMetre::new(25.0);
        let area2 = SquareMetre::new(9.0);
        area -= area2;
        assert_eq!(area, SquareMetre::new(16.0));
    }

    #[test]
    fn test_sub_assign_same_quantity_different_units() {
        let mut area = SquareMetre::new(1.0);
        let area2 = SquareCentimetre::new(2000.0);
        area -= area2;
        assert_eq!(area, SquareMetre::new(0.8));
    }

    #[test]
    fn test_sub_assign_unit_quantity() {
        let mut mass = Kilogram::new(5.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        mass -= mass_quantity;
        assert_eq!(mass, Kilogram::new(3.0));
    }

    #[test]
    fn test_sub_assign_quantity_unit() {
        let mut mass_quantity = Kilogram::new(5.0).into_q();
        let mass_unit = Gram::new(2000.0);
        mass_quantity -= mass_unit;
        assert_eq!(mass_quantity, Mass::new(3.0));
    }

    // ===========================
    // SCALAR MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_scalar_mul_lhs() {
        let scalar = 3.0;
        let force = Newton::new(5.0);
        let result = scalar * force;
        assert_eq!(result, Newton::new(15.0));
    }

    #[test]
    fn test_scalar_mul_rhs() {
        let volume = Litre::new(2.0);
        let scalar = 4.0;
        let result = volume * scalar;
        assert_eq!(result, Litre::new(8.0));
    }

    // ===========================
    // SCALAR MUL_ASSIGN TESTS
    // ===========================

    #[test]
    fn test_scalar_mul_assign() {
        let mut pressure = Pascal::new(100.0);
        pressure *= 2.5;
        assert_eq!(pressure, Pascal::new(250.0));
    }

    // ===========================
    // UNIT × UNIT MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_unit_mul_unit() {
        let mass = Kilogram::new(5.0);
        let acceleration = MetrePerSecondSquared::new(2.0);
        let force = mass * acceleration;
        assert_eq!(force, Force::new(10.0));
    }

    // ===========================
    // UNIT × QUANTITY MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_unit_mul_quantity() {
        let mass = Kilogram::new(5.0);
        let acceleration_quantity = MetrePerSecondSquared::new(2.0).into_q();
        let force = mass * acceleration_quantity;
        assert_eq!(force, Force::new(10.0));
    }

    // ===========================
    // QUANTITY × UNIT MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_quantity_mul_unit() {
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration = MetrePerSecondSquared::new(2.0);
        let force = mass_quantity * acceleration;
        assert_eq!(force, Force::new(10.0));
    }

    // ===========================
    // QUANTITY × QUANTITY MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_quantity_mul_quantity() {
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration_quantity = MetrePerSecondSquared::new(2.0).into_q();
        let force = mass_quantity * acceleration_quantity;
        assert_eq!(force, Force::new(10.0));
    }

    // ===========================
    // SCALAR DIVISION TESTS
    // ===========================

    #[test]
    fn test_scalar_div_lhs() {
        let scalar = 20.0;
        let velocity = MetrePerSecond::new(4.0);
        let result = scalar / velocity;

        type InverseVelocity = Quantity<Z0, N1, P1, Z0, Z0, Z0, Z0>;
        assert_eq!(result, InverseVelocity::new(5.0));
    }

    #[test]
    fn test_scalar_div_rhs() {
        let energy = Joule::new(100.0);
        let scalar = 5.0;
        let result = energy / scalar;
        assert_eq!(result, Joule::new(20.0));
    }

    // ===========================
    // SCALAR DIV_ASSIGN TESTS
    // ===========================

    #[test]
    fn test_scalar_div_assign() {
        let mut frequency = Hertz::new(1000.0);
        frequency /= 10.0;
        assert_eq!(frequency, Hertz::new(100.0));
    }

    // ===========================
    // UNIT ÷ UNIT DIVISION TESTS
    // ===========================

    #[test]
    fn test_unit_div_unit() {
        let force = Newton::new(10.0);
        let mass = Kilogram::new(5.0);
        let acceleration = force / mass;
        assert_eq!(acceleration, Acceleration::new(2.0));
    }

    // ===========================
    // UNIT ÷ QUANTITY DIVISION TESTS
    // ===========================

    #[test]
    fn test_unit_div_quantity() {
        let force = Newton::new(10.0);
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration = force / mass_quantity;
        assert_eq!(acceleration, Acceleration::new(2.0));
    }

    // ===========================
    // QUANTITY ÷ UNIT DIVISION TESTS
    // ===========================

    #[test]
    fn test_quantity_div_unit() {
        let force_quantity = Newton::new(10.0).into_q();
        let mass = Kilogram::new(5.0);
        let acceleration = force_quantity / mass;
        assert_eq!(acceleration, Acceleration::new(2.0));
    }

    // ===========================
    // QUANTITY ÷ QUANTITY DIVISION TESTS
    // ===========================

    #[test]
    fn test_quantity_div_quantity() {
        let force_quantity = Newton::new(10.0).into_q();
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration = force_quantity / mass_quantity;
        assert_eq!(acceleration, Acceleration::new(2.0));
    }
}
