use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys::*;

fn set2vec(s: &Set) -> Vec<JsValue> {
    let mut result = Vec::new();
    s.for_each(&mut |x, _, _| result.push(x));
    return result
}

#[wasm_bindgen_test]
fn add() {
    let set = Set::new(&JsValue::undefined());
    set.add(&100.into());
    assert_eq!(set.size(), 1);
    assert_eq!(set2vec(&set)[0], 100);
}

#[wasm_bindgen_test]
fn clear() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());
    assert_eq!(set.size(), 3);
    set.clear();
    assert_eq!(set.size(), 0);
}

#[wasm_bindgen_test]
fn delete() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    assert!(set.delete(&3.into()));
    assert!(!set.delete(&3.into()));
    assert!(!set.delete(&4.into()));
}

#[wasm_bindgen_test]
fn for_each() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    let v = set2vec(&set);
    assert_eq!(v.len(), 3);
    assert!(v.iter().any(|v| *v == 1));
    assert!(v.iter().any(|v| *v == 2));
    assert!(v.iter().any(|v| *v == 3));
}

#[wasm_bindgen_test]
fn has() {
    let set = Set::new(&JsValue::undefined());
    set.add(&1.into());
    set.add(&2.into());
    set.add(&3.into());

    assert!(set.has(&1.into()));
    assert!(!set.has(&1.3.into()));
}

#[wasm_bindgen_test]
fn new() {
    assert_eq!(Set::new(&JsValue::undefined()).size(), 0);
}

#[wasm_bindgen_test]
fn size() {
    let set = Set::new(&JsValue::undefined());
    assert_eq!(set.size(), 0);
    set.add(&1.into());
    assert_eq!(set.size(), 1);
    set.add(&2.into());
    assert_eq!(set.size(), 2);
    set.add(&3.into());
    assert_eq!(set.size(), 3);
}