use core::str::Chars;
use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DecodedElement {
    Integer(isize),
    String(String),
    List(Vec<DecodedElement>),
    Dictionary(HashMap<String, DecodedElement>)
}

pub fn decode(encoded: &str) -> DecodedElement {
    let mut iterable = encoded.chars().peekable();
    
    match iterable.peek().expect("should never panic") {
        'l' => decode_list(iterable).0,
        'd' => decode_dictionary(iterable).0,
        'i' => decode_int(&mut iterable),
        _ => decode_string(&mut iterable),
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
    println!("{:?}", encoded);

    let size: String = encoded
        .take_while(|char| char.is_digit(10))
        .collect();
    
    let size: usize = size.parse().expect("Invalid string size");
    let string: String = encoded.take(size).collect();

    DecodedElement::String(string)
}

pub fn decode_list(mut encoded: Peekable<Chars>) -> (DecodedElement, Peekable<Chars>) {
    encoded.next().expect("Should not panic.");
    let mut decoded_list: Vec<DecodedElement> = Vec::new();

    while *encoded.peek().expect("Missing list end marker") != 'e' {
        let element = match encoded.peek().expect("Should not panic") {
            'l' => {
                let output = decode_list(encoded);
                encoded = output.1;
                output.0
            },
            'd' => {
                let output = decode_list(encoded);
                encoded = output.1;
                output.0
            },
            'i' => decode_int(&mut encoded),
            _ => decode_string(&mut encoded),
        };

        decoded_list.push(element);
    }

    (DecodedElement::List(decoded_list), encoded)
}

pub fn decode_dictionary(mut encoded: Peekable<Chars>) -> (DecodedElement, Peekable<Chars>) {
    encoded.next().expect("Should not panic.");
    let mut decoded_dict: HashMap<String, DecodedElement> = HashMap::new();

    while *encoded.peek().expect("Missing dict end marker") != 'e' {
        let key = match decode_string(&mut encoded) {
            DecodedElement::String(value) => value,
            _ => panic!("Invalid key"),
        };

        let value = match encoded.peek().expect("Should not panic") {
            'l' => {
                let output = decode_list(encoded);
                encoded = output.1;
                output.0
            },
            'd' => {
                let output = decode_list(encoded);
                encoded = output.1;
                output.0
            },
            'i' => decode_int(&mut encoded),
            _ => decode_string(&mut encoded),
        };

        decoded_dict.insert(key, value);
    }

    (DecodedElement::Dictionary(decoded_dict), encoded)
}
