use ferrunitas::Unit;
use ferrunitas::system::*;

#[cfg(test)]
mod std_operations_tests {
    use super::*;

    // ===========================
    // ADD TESTS
    // ===========================

    #[test]
    fn test_add_same_measure() {
        let mass1 = Kilogram::new(2.0);
        let mass2 = Kilogram::new(3.0);
        let result = mass1 + mass2;
        assert_eq!(result, Kilogram::new(5.0));
    }

    #[test]
    fn test_add_same_quantity_different_measures() {
        let mass1 = Kilogram::new(2.0);
        let mass2 = Gram::new(500.0);
        let result = mass1 + mass2;
        assert_eq!(result, Kilogram::new(2.5));
    }

    #[test]
    fn test_add_measure_quantity() {
        let mass_measure = Kilogram::new(3.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        let result = mass_measure + mass_quantity;
        assert_eq!(result, Kilogram::new(5.0));
    }

    #[test]
    fn test_add_quantity_measure() {
        let mass_quantity = Kilogram::new(3.0).into_q();
        let mass_measure = Gram::new(2000.0);
        let result = mass_quantity + mass_measure;
        assert_eq!(result, Kilogram::new(5.0).into_q());
    }

    // ===========================
    // ADD_ASSIGN TESTS
    // ===========================

    #[test]
    fn test_add_assign_same_measure() {
        let mut length = Metre::new(10.0);
        let length2 = Metre::new(5.0);
        length += length2;
        assert_eq!(length, Metre::new(15.0));
    }

    #[test]
    fn test_add_assign_same_quantity_different_measures() {
        let mut length = Metre::new(1.0);
        let length2 = Centimetre::new(50.0);
        length += length2;
        assert_eq!(length, Metre::new(1.5));
    }

    #[test]
    fn test_add_assign_measure_quantity() {
        let mut mass = Kilogram::new(3.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        mass += mass_quantity;
        assert_eq!(mass, Kilogram::new(5.0));
    }

    #[test]
    fn test_add_assign_quantity_measure() {
        let mut mass_quantity = Kilogram::new(3.0).into_q();
        let mass_measure = Gram::new(2000.0);
        mass_quantity += mass_measure;
        assert_eq!(mass_quantity, Kilogram::new(5.0).into_q());
    }

    // ===========================
    // SUB TESTS
    // ===========================

    #[test]
    fn test_sub_same_measure() {
        let time1 = Second::new(10.0);
        let time2 = Second::new(3.0);
        let result = time1 - time2;
        assert_eq!(result, Second::new(7.0));
    }

    #[test]
    fn test_sub_same_quantity_different_measures() {
        let time1 = Minute::new(2.0);
        let time2 = Second::new(30.0);
        let result = time1 - time2;
        assert_eq!(result, Minute::new(1.5));
    }

    #[test]
    fn test_sub_measure_quantity() {
        let mass_measure = Kilogram::new(5.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        let result = mass_measure - mass_quantity;
        assert_eq!(result, Kilogram::new(3.0));
    }

    #[test]
    fn test_sub_quantity_measure() {
        let mass_quantity = Kilogram::new(5.0).into_q();
        let mass_measure = Gram::new(2000.0);
        let result = mass_quantity - mass_measure;
        assert_eq!(result, Kilogram::new(3.0).into_q());
    }

    // ===========================
    // SUB_ASSIGN TESTS
    // ===========================

    #[test]
    fn test_sub_assign_same_measure() {
        let mut area = SquareMetre::new(25.0);
        let area2 = SquareMetre::new(9.0);
        area -= area2;
        assert_eq!(area, SquareMetre::new(16.0));
    }

    #[test]
    fn test_sub_assign_same_quantity_different_measures() {
        let mut area = SquareMetre::new(1.0);
        let area2 = SquareCentimetre::new(2000.0);
        area -= area2;
        assert_eq!(area, SquareMetre::new(0.8));
    }

    #[test]
    fn test_sub_assign_measure_quantity() {
        let mut mass = Kilogram::new(5.0);
        let mass_quantity = Gram::new(2000.0).into_q();
        mass -= mass_quantity;
        assert_eq!(mass, Kilogram::new(3.0));
    }

    #[test]
    fn test_sub_assign_quantity_measure() {
        let mut mass_quantity = Kilogram::new(5.0).into_q();
        let mass_measure = Gram::new(2000.0);
        mass_quantity -= mass_measure;
        assert_eq!(mass_quantity, Kilogram::new(3.0).into_q());
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
    // measure × measure MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_measure_mul_measure() {
        let mass = Kilogram::new(5.0);
        let acceleration = MetrePerSecondSquared::new(2.0);
        let force = mass * acceleration;
        assert_eq!(force, Newton::new(10.0).into_q());
    }

    // ===========================
    // measure × QUANTITY MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_measure_mul_quantity() {
        let mass = Kilogram::new(5.0);
        let acceleration_quantity = MetrePerSecondSquared::new(2.0).into_q();
        let force = mass * acceleration_quantity;
        assert_eq!(force, Newton::new(10.0).into_q());
    }

    // ===========================
    // QUANTITY × measure MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_quantity_mul_measure() {
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration = MetrePerSecondSquared::new(2.0);
        let force = mass_quantity * acceleration;
        assert_eq!(force, Newton::new(10.0).into_q());
    }

    // ===========================
    // QUANTITY × QUANTITY MULTIPLICATION TESTS
    // ===========================

    #[test]
    fn test_quantity_mul_quantity() {
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration_quantity = MetrePerSecondSquared::new(2.0).into_q();
        let force = mass_quantity * acceleration_quantity;
        assert_eq!(force, Newton::new(10.0).into_q());
    }

    // ===========================
    // SCALAR DIVISION TESTS
    // ===========================

    #[test]
    fn test_scalar_div_lhs() {
        let scalar = 20.0;
        let velocity = Metre::new(4.0);
        let result = scalar / velocity;

        assert_eq!(result, ReciprocalMetre::new(5.0).into_q());
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
    // measure ÷ measure DIVISION TESTS
    // ===========================

    #[test]
    fn test_measure_div_measure() {
        let force = Newton::new(10.0);
        let mass = Kilogram::new(5.0);
        let acceleration = force / mass;
        assert_eq!(acceleration, MetrePerSecondSquared::new(2.0).into_q());
    }

    // ===========================
    // measure ÷ QUANTITY DIVISION TESTS
    // ===========================

    #[test]
    fn test_measure_div_quantity() {
        let force = Newton::new(10.0);
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration = force / mass_quantity;
        assert_eq!(acceleration, MetrePerSecondSquared::new(2.0).into_q());
    }

    // ===========================
    // QUANTITY ÷ measure DIVISION TESTS
    // ===========================

    #[test]
    fn test_quantity_div_measure() {
        let force_quantity = Newton::new(10.0).into_q();
        let mass = Kilogram::new(5.0);
        let acceleration = force_quantity / mass;
        assert_eq!(acceleration, MetrePerSecondSquared::new(2.0).into_q());
    }

    // ===========================
    // QUANTITY ÷ QUANTITY DIVISION TESTS
    // ===========================

    #[test]
    fn test_quantity_div_quantity() {
        let force_quantity = Newton::new(10.0).into_q();
        let mass_quantity = Kilogram::new(5.0).into_q();
        let acceleration = force_quantity / mass_quantity;
        assert_eq!(acceleration, MetrePerSecondSquared::new(2.0).into_q());
    }
}
