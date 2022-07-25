#[cfg(test)]
use assert_json_diff::assert_json_eq;
use decart::{from_file, OctoCart};
use serde_json::{json, Value};

#[test]
fn minimal_from_file() {
    let cart: OctoCart = from_file("tests/test_carts/minimal.gif").unwrap();
    let cart_string = &cart.to_string();
    let json: Value = serde_json::from_str(cart_string).unwrap();
    assert_json_eq!(
        json,
        json!({"program":": main","options":{"tickrate":7,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":false,"loadStoreQuirks":false,"vfOrderQuirks":false,"clipQuirks":true,"vBlankQuirks":true,"jumpQuirks":false,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":true,"fontStyle":"octo"}})
    );
}
