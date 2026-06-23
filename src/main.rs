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
        .pop_front()
        .unwrap();

    let b = stack
        .pop_front()
        .unwrap();

    Some((a, b))
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

        print_stack(&stack);

        print!("> ");
        io::stdout().flush();

        io::stdin()
            .read_line(&mut s)
            .expect("Couldn't read line!");

        let token_stream: parsing::TokenStream = match s.parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Invalid input!");
                continue;
            },
        };

        for token in token_stream.tokens() {
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
            }
        }
    }
}
