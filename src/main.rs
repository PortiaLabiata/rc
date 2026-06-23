mod operations;
mod parsing;

use std::io;

fn main() {
    loop {
        let mut s = String::new();
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
            println!("{:?}", token);
        }
    }
}
