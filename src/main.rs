mod operations;
mod parsing;

use std::io;
use std::collections::VecDeque;
use std::io::Write;
use parsing::Token;
use operations::Number;

fn pop2(stack: &mut VecDeque<Number>) -> Option<(Number, Number)> {
    if stack.len() < 2 {
        return None;
    }

    let a = stack
        .pop_back()
        .unwrap();

    let b = stack
        .pop_back()
        .unwrap();

    Some((b, a))
}

fn print_stack(stack: &VecDeque<Number>) {
    for v in stack.iter() {
        print!("{} ", v);
    }
    println!("");
}

fn main() {
    let mut stack = VecDeque::new();

    loop {
        let mut s = String::new();

        print!("> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut s)
            .expect("Couldn't read line!");

        let token_stream: parsing::TokenStream = match s.parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid token: {}!", e);
                continue;
            },
        };

        for token in token_stream.iter() {
            match token {
                Token::Number(v) => stack.push_back(v.clone()),
                Token::OpBin(o) => {
                    let (a, b) = match pop2(&mut stack) {
                        Some(v) => v,
                        None => {
                            eprintln!("Not enough values on stack!");
                            break;
                        }
                    };

                    match o.apply(a, b) {
                        Some(v) => stack.push_back(v),
                        None => {
                            eprintln!("Invalid operation!");
                            break;
                        }
                    }
                },
                Token::OpUn(o) => {
                    let a = match stack.pop_back() {
                        Some(v) => v,
                        None => {
                            eprintln!("Not enough values on stack!");
                            break;
                        }
                    };

                    match o.apply(a) {
                        Some(v) => stack.push_back(v),
                        None => {
                            eprintln!("Invalid operation!");
                            break;
                        }
                    }
                }
            }
        }

        print_stack(&stack);
    }
}
