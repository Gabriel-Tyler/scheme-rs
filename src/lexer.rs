use anyhow::Result;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        f.write_str(
            match self {
                Integer(n) => format!("{}", n),
                Symbol(s) => format!("{}", s),
                LParen => format!("("),
                RParen => format!(")"),
            }
            .as_str(),
        )
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    source
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| match s {
            "(" => Token::LParen,
            ")" => Token::RParen,
            _ => {
                if let Ok(x) = s.parse::<i64>() {
                    Token::Integer(x)
                } else {
                    Token::Symbol(s.to_owned())
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let tokens = tokenize("(+ 4 8)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_owned()),
                Token::Integer(4),
                Token::Integer(8),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_circle_area() {
        let program = "
            (
                (define pi 314)
                (define r 10)
                (* pi (* r r))
            )
        ";
        let tokens = tokenize(program);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_owned()),
                Token::Symbol("pi".to_owned()),
                Token::Integer(314),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_owned()),
                Token::Symbol("r".to_owned()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_owned()),
                Token::Symbol("pi".to_owned()),
                Token::LParen,
                Token::Symbol("*".to_owned()),
                Token::Symbol("r".to_owned()),
                Token::Symbol("r".to_owned()),
                Token::RParen,
                Token::RParen,
                Token::RParen,
            ]
        );
    }
}
