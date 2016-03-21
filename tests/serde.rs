#![cfg(feature = "serde_impl")]

extern crate linear_map;
extern crate serde;
extern crate serde_json;

use linear_map::LinearMap;

#[test]
fn test_ser_empty() {
    let map = LinearMap::<String, u32>::new();
    let j = serde_json::to_string(&map).unwrap();
    let expected = "{}";
    assert_eq!(j, expected);
}

#[test]
fn test_ser() {
    let mut map = LinearMap::new();
    map.insert("b", 20);
    map.insert("a", 10);
    map.insert("c", 30);

    let j = serde_json::to_string(&map).unwrap();
    let expected = r#"{"b":20,"a":10,"c":30}"#;
    assert_eq!(j, expected);
}

#[test]
fn test_de_empty() {
    let j = "{}";
    let map: LinearMap<String, u32> = serde_json::from_str(j).unwrap();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_de() {
    let j = r#"{"b":20,"a":10,"c":30}"#;
    let map: LinearMap<String, u32> = serde_json::from_str(j).unwrap();
    let items: Vec<_> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    assert_eq!(items, [("b".to_owned(), 20),
                       ("a".to_owned(), 10),
                       ("c".to_owned(), 30)]);
}
