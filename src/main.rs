extern crate clap;

mod lexer;
mod module;
mod parser;
mod token;
mod utils;

use lexer::Lexer;
use module::Module;
use parser::Parser;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("mycc")
        .version("0.1.0")
        .author("tkclimb")
        .about("mycc (MY C Compiler)")
        .arg(Arg::with_name("source_file").required(true))
        .get_matches();

    let source_file_path = matches
        .value_of("source_file")
        .expect("source file missing...");

    println!("source file path: {}", source_file_path);

    let contents = std::fs::read_to_string(source_file_path).expect("[error] read_to_string");
    let mut lexer = Lexer::new(contents.chars().collect());
    let token_list = lexer.tokenize();
    let mut parser = Parser::new(token_list);
    let module = parser.parse();
    println!("{}", module);

    Ok(())
}
