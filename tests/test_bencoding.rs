use core::panic;
use std::collections::HashMap;

use torrent_rust::bencoding::{self, DecodedElement};

fn example_list() -> Vec<DecodedElement> {
    let a = DecodedElement::Integer(1);
    let b = DecodedElement::Integer(2);
    let c = DecodedElement::Integer(3);

    vec![a, b, c]
}

#[test]
fn test_decode_string() {
    let input = "20:37355668913929142336";
    let expected = "37355668913929142336";

    let result = bencoding::decode(input);
    match result {
        DecodedElement::String(value) => assert_eq!(expected, value),
        _ => panic!("Wrong type returned"),
    }
}

#[test]
fn test_decode_int() {
    let input = "i69e";
    let expected = 69;

    let result = bencoding::decode(input);
    match result {
        DecodedElement::Integer(value) => assert_eq!(expected, value),
        _ => panic!("Wrong type returned"),
    }
}

#[test]
fn test_decode_list() {
    let input = "li34e4:spam4:eggsli1ei2ei3eee";

    let expected = vec![
        DecodedElement::Integer(34),
        DecodedElement::String("spam".to_string()),
        DecodedElement::String("eggs".to_string()),
        DecodedElement::List(example_list()),
    ];

    let result = bencoding::decode(input);
    match result {
        DecodedElement::List(value) => assert_eq!(expected, value),
        _ => panic!("Wrong type returned"),
    }
}

#[test]
fn test_decode_dictionary() {
    let input = "d3:cow3:moo4:spami34e4:eggsli1ei2ei3eee";

    let mut expected: HashMap<String, DecodedElement> = HashMap::new();
    expected.insert("cow".to_string(), DecodedElement::String("moo".to_string()));
    expected.insert("spam".to_string(), DecodedElement::Integer(34));
    expected.insert("eggs".to_string(), DecodedElement::List(example_list()));

    let result = bencoding::decode(input);
    match result {
        DecodedElement::Dictionary(value) => assert_eq!(expected, value),
        _ => panic!("Wrong type returned"),
    }
}
