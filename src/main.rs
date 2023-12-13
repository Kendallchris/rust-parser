/* 
    This module is the main which handles the CLI (argument checks) and links together all other modules
    (lexer.rs, parser.rs, scheme.rs, and prolog.rs)
    Chris Kendall
    15 October 2023
*/

mod lexer;
mod parser;
mod scheme;
mod prolog;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <filename> [-s|-p]");
        return;
    }

    let filename = &args[1];
    let flag = &args[2];

    // check for propper 3rd arg
    let is_scheme_output = match flag.as_str() {
        "-s" => true,
        "-p" => false,
        _ => {
            println!("Invalid flag. Use -s for Scheme or -p for Prolog.");
            return;
        }
    };

    let content = fs::read_to_string(filename).expect("Failed to read file");

    // call lexical analysis
    let tokens_parsed = match lexer::lexical_analysis(&content) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("Lexical error: {}", e);
            return;
        }
    };

    // call syntactical analysis
    let mut parser = parser::Parser::new(tokens_parsed);
    let tokens_parsed = match parser.parse() {
        Ok(nodes) => {
            println!("Lexical and Syntax analysis passed");
            nodes
        },
        Err(e) => {
            println!("Parsing error: {}", e);
            return;
        }
    };

    // output the desired language
    if is_scheme_output {
        let scheme_output = tokens_parsed.iter()
            .map(|node| scheme::convert_to_scheme(node))
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", scheme_output);
    } else {
        let prolog_statements = tokens_parsed.iter()
            .map(|node| prolog::convert_to_prolog(node))
            .filter(|s| !s.is_empty())  // Filter out empty strings
            .collect::<Vec<String>>()
            .join(",\n   ");
        
        // Remove trailing comma if present and format the output
        let prolog_output = if prolog_statements.ends_with(",") {
            format!("main :-\n   {}.", &prolog_statements[..prolog_statements.len()-1])
        } else {
            format!("main :-\n   {}.", prolog_statements)
        };
        
        println!("{}", prolog_output);
    }
}
