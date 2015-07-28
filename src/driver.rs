use std::io;
use std::io::Write;

//use builder::*;
use lexer::*;
use parser::*;

pub use self::Stage::{
    Exec,
    IR,
    AST,
    Tokens
};

//< parser-stage
#[derive(PartialEq, Clone, Debug)]
pub enum Stage {
//> parser-stage
    Exec,
    IR,
//< parser-stage
    AST,
    Tokens
}
//> parser-stage

//< parser-driver
pub fn main_loop(stage: Stage) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut parser_settings = default_parser_settings();
//> parser-driver
    //let mut context = Context::new("main");
//< parser-driver

    'main: loop {
        print!("> ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).ok().expect("Failed to read line");
        if input.as_str() == ".quit\n" {
            break;
        }

        // the constructed AST
        let mut ast = Vec::new();
        // tokens left from the previous lines
        let mut prev = Vec::new();
        loop {
            let tokens = tokenize(input.as_str());
            if stage == Tokens {
                println!("{:?}", tokens);
                continue 'main
            }

            prev.extend(tokens.into_iter());

            let parsing_result = parse(prev.as_slice(), ast.as_slice(), &mut parser_settings);
            match parsing_result {
                Ok((parsed_ast, rest)) => {
                    ast.extend(parsed_ast.into_iter());
                    if rest.is_empty() {
                        // we have parsed a full expression
                        break
                    } else {
                        prev = rest;
                    }
                },
                Err(message) => {
                    println!("Error occured: {}", message);
                    continue 'main
                }
            }
            print!(". ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).ok().expect("Failed to read line");
        }

        if stage == AST {
            println!("{:?}", ast);
            continue
        }
//> parser-driver

        /*match ast.codegen(&mut context) {
            Ok((value, runnable)) => if runnable && stage == Exec {
                println!("=> {}", run(value, &context))
            } else {
                dump_value(value)
            },
            Err(message) => println!("Error occured: {}", message)
        }*/
//< parser-driver
    }
//> parser-driver

    /*if stage == IR || stage == Exec {
        context.dump();
    }*/
//< parser-driver
}
//> parser-driver
