use core::panic;

use torrent_rust::bencoding;

#[test]
fn test_decode_string() {
    let input = "20:37355668913929142336";
    let expected = "37355668913929142336";

    let (result, _) = bencoding::decode_string(input);
    match result {
        bencoding::DecodedElement::String(value) => assert_eq!(expected, value),
        _ => panic!("Wrong type returned"),
    }
}

#[test]
fn test_decode_int() {
    let input = "i69e";
    let expected = 69;

    let (result, _) = bencoding::decode_int(input);
    match result {
        bencoding::DecodedElement::Integer(value) => assert_eq!(expected, value),
        _ => panic!("Wrong type returned"),
    }
}

#[test]
fn test_decode_list() {
    let input = "li34e4:spam4:eggsi54ee";

    let expected = vec![
        bencoding::DecodedElement::Integer(34),
        bencoding::DecodedElement::String("spam".to_string()),
        bencoding::DecodedElement::String("eggs".to_string()),
        bencoding::DecodedElement::Integer(54),
    ];

    let (result, _) = bencoding::decode_list(input);
    match result {
        bencoding::DecodedElement::List(value) => assert_eq!(expected, value),
        _ => panic!("Wrong type returned"),
    }
}

#[test]
fn test_decode_dictionary() {
    assert!(true);
}
