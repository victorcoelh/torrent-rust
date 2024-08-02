use std::env;
use torrent_rust::cmd_parser::ConsoleParser;

fn main() {
    let mut args = env::args();
    let parser = ConsoleParser::new(&mut args);

    parser.process_command();
}
