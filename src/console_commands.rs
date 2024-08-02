use std::str::FromStr;

pub enum ConsoleCommand {
    Decode,
}

impl FromStr for ConsoleCommand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "decode" => Ok(ConsoleCommand::Decode),
            _ => Err(input.to_string()),
        }
    }
}
