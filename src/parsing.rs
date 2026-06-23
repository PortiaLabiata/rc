use std::str::FromStr;

use crate::operations::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Number(Number),
    OpBin(OpBin),
    OpUn(OpUn),
}

impl From<Number> for Token {
    fn from(v: Number) -> Self {
        Token::Number(v)
    }
}

impl From<OpBin> for Token {
    fn from(v: OpBin) -> Self {
        Token::OpBin(v)
    }
}

impl From<OpUn> for Token {
    fn from(v: OpUn) -> Self {
        Token::OpUn(v)
    }
}

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.tokens.iter()
    }
}

impl FromStr for TokenStream {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .split_whitespace();

        let mut result = Vec::new();
        for substr in s {
            let token;
            if let Ok(v) = substr.parse::<i64>() {
                token = Token::from(Number::Int(v));
            } else if let Ok(v) = substr.parse::<f64>() {
                token = Token::from(Number::Float(v));
            } else if let Ok(v) = substr.parse::<OpBin>() {
                token = Token::from(v);
            } else if let Ok(v) = substr.parse::<OpUn>() {
                token = Token::from(v); 
            } else {
                return Err(());
            }
            result.push(token);
        }

        Ok(TokenStream { tokens: result })
    }
}
