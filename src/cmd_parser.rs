use std::str::FromStr;

use crate::console_commands::ConsoleCommand;
use crate::bencoding::{self, DecodedElement};

pub struct ConsoleParser {
    command: ConsoleCommand,
    value: String,
}

impl ConsoleParser {
    pub fn new(args: &mut impl Iterator<Item = String>) -> Self {
        args.next();

        let command = args
            .next()
            .expect("Missing command");
        let command = ConsoleCommand::from_str(&command)
            .expect("Invalid Command");

        let value = args
            .next()
            .expect("Missing encoded string");

        ConsoleParser {
            command,
            value,
        }
    }

    pub fn process_command(&self) {
        match self.command {
            ConsoleCommand::Decode => {
                let (decoded, _size) = bencoding::decode(&self.value);

                match decoded {
                    DecodedElement::String(value) => println!("{}", value),
                    DecodedElement::Integer(value) => println!("{}", value),
                    DecodedElement::List(value) => println!("{:?}", value),
                    _ => (),
                }
            }
        };
    }
}
