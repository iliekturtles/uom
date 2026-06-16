//! Tests for unit `serialize` and `deserialize` functions.

// We re-export `serde_core` as `serde` in `lib.rs`, which shadows the `serde` crate.
extern crate serde as serde_with_derive;

storage_types! {
    types: Float, PrimInt;

    use crate::tests::*;
    use super::serde_with_derive::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Component {
        length: length::Length<U<V>, V>,

        #[serde(serialize_with = "length::meter::serialize")]
        #[serde(deserialize_with = "length::meter::deserialize")]
        length_m: length::Length<U<V>, V>,

        #[serde(serialize_with = "length::kilometer::serialize")]
        #[serde(deserialize_with = "length::kilometer::deserialize")]
        length_km: length::Length<U<V>, V>,
    }

    #[test]
    fn deserialize() {
        let json = r#"{ "length": 2000, "length_m": 3000, "length_km": 5 }"#;
        let component: Component = serde_json::from_str(json).unwrap();

        Test::assert_eq(&V::from_f64(2000.0).unwrap(), &component.length.get::<meter>());
        Test::assert_eq(&V::from_f64(2.0).unwrap(), &component.length.get::<kilometer>());
        Test::assert_eq(&V::from_f64(3000.0).unwrap(), &component.length_m.get::<meter>());
        Test::assert_eq(&V::from_f64(3.0).unwrap(), &component.length_m.get::<kilometer>());
        Test::assert_eq(&V::from_f64(5.0).unwrap(), &component.length_km.get::<kilometer>());
        Test::assert_eq(&V::from_f64(5000.0).unwrap(), &component.length_km.get::<meter>());
    }

    #[test]
    fn serialize() {
        let component = Component {
            length: length::Length::new::<meter>(V::from_f64(2000.0).unwrap()),
            length_m: length::Length::new::<meter>(V::from_f64(3000.0).unwrap()),
            length_km: length::Length::new::<kilometer>(V::from_f64(5.0).unwrap()),
        };

        let json = serde_json::to_value(&component).unwrap();

        assert_eq!(json["length"], serde_json::to_value(V::from_f64(2000.0).unwrap()).unwrap());
        assert_eq!(json["length_m"], serde_json::to_value(V::from_f64(3000.0).unwrap()).unwrap());
        assert_eq!(json["length_km"], serde_json::to_value(V::from_f64(5.0).unwrap()).unwrap());
    }
}
