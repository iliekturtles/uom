#![cfg(all(feature = "serde", feature = "i64"))]

use crate::tests::*;
use ::serde::{Deserialize, Serialize};

/// Test struct using f64 storage with custom unit deserializers and serializers
#[derive(Debug, Serialize, Deserialize)]
struct ComponentF64 {
    length: length::Length<U<f64>, f64>,

    #[serde(serialize_with = "length::meter::serialize")]
    #[serde(deserialize_with = "length::meter::deserialize")]
    length_m: length::Length<U<f64>, f64>,

    #[serde(serialize_with = "length::kilometer::serialize")]
    #[serde(deserialize_with = "length::kilometer::deserialize")]
    length_km: length::Length<U<f64>, f64>,
}

/// Test struct using f32 storage with custom unit deserializers and serializers
#[derive(Debug, Serialize, Deserialize)]
struct ComponentF32 {
    length: length::Length<U<f32>, f32>,

    #[serde(serialize_with = "length::meter::serialize")]
    #[serde(deserialize_with = "length::meter::deserialize")]
    length_m: length::Length<U<f32>, f32>,

    #[serde(serialize_with = "length::kilometer::serialize")]
    #[serde(deserialize_with = "length::kilometer::deserialize")]
    length_km: length::Length<U<f32>, f32>,
}

/// Test struct using i64 storage with custom unit deserializers and serializers
#[derive(Debug, Serialize, Deserialize)]
struct ComponentI64 {
    length: length::Length<U<i64>, i64>,

    #[serde(serialize_with = "length::meter::serialize")]
    #[serde(deserialize_with = "length::meter::deserialize")]
    length_m: length::Length<U<i64>, i64>,

    #[serde(serialize_with = "length::kilometer::serialize")]
    #[serde(deserialize_with = "length::kilometer::deserialize")]
    length_km: length::Length<U<i64>, i64>,
}

#[test]
fn deserialize_with_different_units() {
    let json = r#"{ "length": 250, "length_m": 100, "length_km": 5 }"#;

    let component: ComponentF64 = serde_json::from_str(json).unwrap();
    assert_eq!(component.length.get::<meter>(), 250.0);
    assert_eq!(component.length.get::<kilometer>(), 0.25);
    assert_eq!(component.length_m.get::<meter>(), 100.0);
    assert_eq!(component.length_m.get::<kilometer>(), 0.1);
    assert_eq!(component.length_km.get::<kilometer>(), 5.0);
    assert_eq!(component.length_km.get::<meter>(), 5000.0);

    let component: ComponentF32 = serde_json::from_str(json).unwrap();
    assert_eq!(component.length.get::<meter>(), 250.0);
    assert_eq!(component.length.get::<kilometer>(), 0.25);
    assert_eq!(component.length_m.get::<meter>(), 100.0);
    assert_eq!(component.length_m.get::<kilometer>(), 0.1);
    assert_eq!(component.length_km.get::<kilometer>(), 5.0);
    assert_eq!(component.length_km.get::<meter>(), 5000.0);

    let component: ComponentI64 = serde_json::from_str(json).unwrap();
    assert_eq!(component.length.get::<meter>(), 250);
    assert_eq!(component.length.get::<kilometer>(), 0);
    assert_eq!(component.length_m.get::<meter>(), 100);
    assert_eq!(component.length_m.get::<kilometer>(), 0);
    assert_eq!(component.length_km.get::<kilometer>(), 5);
    assert_eq!(component.length_km.get::<meter>(), 5000);
}

#[test]
fn serialize_with_different_units() {
    use length::Length;

    let component = ComponentF64 {
        length: Length::new::<meter>(250.0),
        length_m: Length::new::<kilometer>(0.1),
        length_km: Length::new::<kilometer>(5.0),
    };

    let json = serde_json::to_string(&component).unwrap();
    assert_eq!(json, r#"{"length":250.0,"length_m":100.0,"length_km":5.0}"#);

    let component = ComponentF32 {
        length: Length::new::<meter>(250.0),
        length_m: Length::new::<kilometer>(0.1),
        length_km: Length::new::<kilometer>(5.0),
    };

    let json = serde_json::to_string(&component).unwrap();
    assert_eq!(json, r#"{"length":250.0,"length_m":100.0,"length_km":5.0}"#);

    let component = ComponentI64 {
        length: Length::new::<meter>(250),
        length_m: Length::new::<kilometer>(1),
        length_km: Length::new::<kilometer>(5),
    };

    let json = serde_json::to_string(&component).unwrap();
    assert_eq!(json, r#"{"length":250,"length_m":1000,"length_km":5}"#);
}
