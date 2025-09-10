//! Support for (de)serialization of `Quantity` with Serde.
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::dimension::Dimensioned;
use crate::model::measure::Measure;
use crate::model::quantity::{Quantity, QuantityTag};
use crate::model::unit::Unit;

/// Internal quantity type for serialization only.
/// Converts all type params into runtime values.
#[derive(Serialize, Deserialize)]
struct RuntimeQuantity<'a> {
    value: f64,
    dimensional_vector: [i8; 7],
    tag: &'a str,
}

/// Internal measure type for serialization only.
/// Converts all type params into runtime values.
#[derive(Serialize, Deserialize)]
struct RuntimeMeasure<'a> {
    value: f64,
    unit: &'a str,
}

// ===================================================================================
// SERDE IMPLEMENTATIONS
// ===================================================================================

/// Implement Serialize for any Quantity
impl<D: Dimensioned, Tag: QuantityTag> Serialize for Quantity<D, Tag> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let dims = D::to_array();

        let rtq = RuntimeQuantity {
            value: self.value,
            dimensional_vector: dims,
            tag: Tag::name(),
        };
        rtq.serialize(serializer)
    }
}

/// Implement Deserialize for any Quantity
impl<'de, D: Dimensioned, Tag: QuantityTag> Deserialize<'de> for Quantity<D, Tag> {
    fn deserialize<DS: Deserializer<'de>>(deserializer: DS) -> Result<Self, DS::Error> {
        let rtq = RuntimeQuantity::deserialize(deserializer)?;
        let expected = D::to_array();

        if rtq.dimensional_vector != expected {
            return Err(DS::Error::custom(format!(
                "Dimension mismatch: expected {:?}, got {:?}",
                expected, rtq.dimensional_vector
            )));
        }

        Ok(Quantity::new(rtq.value))
    }
}

/// Implement Serialize for any Measure
impl<U: Unit> Serialize for Measure<U> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let rtm = RuntimeMeasure {
            value: self.value(),
            unit: std::any::type_name::<U>(),
        };
        rtm.serialize(serializer)
    }
}

/// Implement Deserialize for any Measure
impl<'de, U: Unit> Deserialize<'de> for Measure<U> {
    fn deserialize<DS: Deserializer<'de>>(deserializer: DS) -> Result<Self, DS::Error> {
        let rtm = RuntimeMeasure::deserialize(deserializer)?;
        let expected = std::any::type_name::<U>();

        if rtm.unit != expected {
            return Err(DS::Error::custom(format!(
                "Unit mismatch: expected '{}', got '{}'",
                expected, rtm.unit
            )));
        }

        Ok(Measure::new(rtm.value))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::measure::Measure;
    use crate::model::unit::Unit;
    use crate::system::*;

    #[test]
    fn test_quantity_serde() {
        let f = Force::new(42.0);

        // Verify serialization (json here)
        let json = serde_json::to_string(&f).unwrap();
        assert_eq!(
            json,
            "{\"value\":42.0,\"dimensional_vector\":[1,1,-2,0,0,0,0],\"tag\":\"()\"}"
        );

        // Verify restoration
        let f2: Force = serde_json::from_str(&json).unwrap();
        assert_eq!(f, f2);

        // Verify restoration in wrong type fails
        let err: Result<Energy, _> = serde_json::from_str(&json);
        assert!(err.is_err());
    }

    #[test]
    #[cfg(feature = "quantity_tags")]
    fn test_quantity_serde_withtag() {
        let f = SolidAngle::new(42.0);

        // Verify serialization (json here)
        let json = serde_json::to_string(&f).unwrap();
        assert_eq!(
            json,
            "{\"value\":42.0,\"dimensional_vector\":[0,0,0,0,0,0,0],\"tag\":\"SolidAngle\"}"
        );
    }

    #[test]
    fn test_measure_serde() {
        let f = Kilonewton::new(42.0);

        // Verify serialization (json here)
        let json = serde_json::to_string(&f).unwrap();
        assert_eq!(
            json,
            "{\"value\":42.0,\"unit\":\"ferrunitas::system::defs::mechanics::Kilonewton\"}"
        );

        // Verify restoration
        let f2: Measure<Kilonewton> = serde_json::from_str(&json).unwrap();
        assert_eq!(f, f2);

        // Verify restoration in wrong type fails
        let err: Result<Measure<Milliwatt>, _> = serde_json::from_str(&json);
        assert!(err.is_err());
    }
}
