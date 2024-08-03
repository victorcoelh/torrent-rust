use core::str::Chars;
use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DecodedElement {
    Integer(isize),
    String(String),
    List(Vec<DecodedElement>),
    Dictionary(HashMap<String, String>)
}

// TODO: refactor everything to use iterators
pub fn decode(encoded: &str) -> DecodedElement {
    let mut iterable = encoded.chars().peekable();
    
    match iterable.peek().expect("should never panic") {
        'l' => decode_list(iterable),
        'd' => decode_dictionary(iterable),
        _ => decode_from_iter(&mut iterable)
    }
}

pub fn decode_from_iter(iterator: &mut Peekable<Chars>) -> DecodedElement {
    match iterator.peek().expect("Should never panic") {
        'i' => decode_int(iterator),
        _ => decode_string(iterator),
    }
}

pub fn decode_int(encoded: &mut Peekable<Chars>) -> DecodedElement {
    let number: String = encoded.skip(1)
        .take_while(|c| c.is_digit(10))
        .collect();

    let number: isize = number.parse().expect("Invalid integer inserted");
    DecodedElement::Integer(number)
}

// TODO: throw error when the string size doesn't match the actual size of the string.
pub fn decode_string(encoded: &mut Peekable<Chars>) -> DecodedElement {
    let size: String = encoded
        .take_while(|char| char.is_digit(10))
        .collect();
    
    let size: usize = size.parse().expect("Invalid string size");
    let string: String = encoded.take(size).collect();

    DecodedElement::String(string)
}

// TODO: Improve error handling
pub fn decode_list(mut encoded: Peekable<Chars>) -> DecodedElement {
    let mut decoded_list: Vec<DecodedElement> = Vec::new();
    encoded.next().expect("should not panic");

    while *encoded.peek().expect("List missing an end marker") != 'e' {
        let element = decode_from_iter(&mut encoded);
        decoded_list.push(element);
    };
    DecodedElement::List(decoded_list)
}

pub fn decode_dictionary(mut encoded: Peekable<Chars>) -> DecodedElement {
    let mut decoded_dict: HashMap<String, String> = HashMap::new();
    encoded.next().expect("should not panic");

    while *encoded.peek().expect("List missing an end marker") != 'e' {
        let key = get_dict_item(decode_from_iter(&mut encoded));
        let value = get_dict_item(decode_from_iter(&mut encoded));

        decoded_dict.insert(key, value);
    }
    DecodedElement::Dictionary(decoded_dict)
}

fn get_dict_item(element: DecodedElement) -> String {
    match element {
        DecodedElement::String(value) => value,
        _ => panic!("Invalid dict item: Dictionary should only contain strings."),
    }
}
