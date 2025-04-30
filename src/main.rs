use std::{env, fs};

pub mod ast;
pub mod tokens;

mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

mod interpreter;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[Usage]\n{} [source_file]", args[0]);
        return;
    }

    let source = {
        if let Ok(source) = fs::read_to_string(&args[1]) {
            source
        } else {
            println!("[Error]\nCould not open file");
            return;
        }
    };

    let mut lexer = Lexer::new(source, args[1].clone());
    lexer.lex();

    // dbg!(&lexer.output_tokens);

    let mut parser = Parser::new(lexer.output_tokens);
    parser.parse();

    // dbg!(&parser.output_nodes);

    let mut interpreter = Interpreter::new(parser.output_nodes);
    interpreter.interpret();
}
