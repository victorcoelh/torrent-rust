use core::panic;
use std::collections::HashMap;

#[derive(Debug)]
pub enum DecodedElement {
    Integer(isize),
    String(String),
    List(Vec<DecodedElement>),
    Dictionary(HashMap<String, String>)
}

// TODO: refactor everything to use iterators
pub fn decode(encoded: &str) -> (DecodedElement, usize) {
    match encoded.chars().next().expect("should never panic") {
        'i' => decode_int(encoded),
        'l' => decode_list(encoded),
        'd' => panic!("Not Implemented"),
        _ => decode_string(encoded),
    }
}

pub fn decode_int(encoded_value: &str) -> (DecodedElement, usize) {
    let end_marker = encoded_value.find("e").expect("Missing int 'e' marker");

    let number = &encoded_value[1..end_marker];
    let number: isize = number.parse().expect("The string received is not an integer");

    (DecodedElement::Integer(number), end_marker+2)
}

// TODO: throw error when the string size doesn't match the actual size of the string.
pub fn decode_string(encoded_value: &str) -> (DecodedElement, usize) {
    let size: String = encoded_value
        .chars()
        .take_while(|char| *char != ':')
        .collect();

    let start_marker = size.len();
    println!("{}", size);
    let size: usize = size.parse().expect("Invalid string length");

    let decoded_string = encoded_value
        .chars()
        .skip(start_marker+1)
        .take(size)
        .collect();

    (DecodedElement::String(decoded_string), size+2)
}

// TODO: Improve error handling
pub fn decode_list(encoded_value: &str) -> (DecodedElement, usize) {
    let encoded_string = encoded_value.to_string();
    let mut decoded_list: Vec<DecodedElement> = Vec::new();
    let mut total_size = 1;

    while encoded_string.chars().skip(total_size).next().unwrap() != 'e' {
        let (element, skip) = decode(&encoded_string[total_size..]);
        decoded_list.push(element);

        total_size += skip;
    };

    (DecodedElement::List(decoded_list), total_size)
}
