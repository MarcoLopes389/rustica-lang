use std::iter::Peekable;
use std::str::Chars;
use crate::frontend::ast::{Token, TokenType};

fn gen_token(token_type: TokenType, value: String) -> Token {
    Token {
        kind: token_type,
        value
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn is_eof(c: char) -> bool {
    c == '\n' || c == '\r'
}

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

fn build_number(src: &mut Peekable<Chars>, first_char: char) -> String {
    let mut number_str = first_char.to_string();
    number_str.push_str(consume_while(src, is_digit).as_str());

    if let Some(&'.') = src.peek() {
        src.next();
        number_str.push('.');
        number_str.push_str(consume_while(src, is_digit).as_str());
    }
    number_str
}

fn build_identifier(src: &mut Peekable<Chars>, first_char: char) -> String {
    let mut identifier_str = first_char.to_string();
    identifier_str.push_str(consume_while(src, is_alphanumeric).as_str());
    identifier_str
}

fn next_token(src: &mut Peekable<Chars>) -> Option<Token> {
    let current_char = src.next()?;
    
    match current_char {
        '*' | '/' | '-' | '+' | '%' => Some(gen_token(TokenType::Binary, current_char.to_string())),
        '(' => Some(gen_token(TokenType::OpenParen, current_char.to_string())),
        ')' => Some(gen_token(TokenType::CloseParen, current_char.to_string())),
        '{' => Some(gen_token(TokenType::OpenBracket, current_char.to_string())),
        '}' => Some(gen_token(TokenType::CloseBracket, current_char.to_string())),
        '.' => Some(gen_token(TokenType::Dot, current_char.to_string())),
        ';' => Some(gen_token(TokenType::Comma, current_char.to_string())),
        '"' => {
            let literal = consume_while(src, |c| c != '"');
            if src.next().is_none() {
                panic!("Unterminated string literal");
            }
            Some(gen_token(TokenType::String, literal))
        },
        '=' => {
            if let Some(&'=') = src.peek() {
                src.next();
                Some(gen_token(TokenType::EqualsEquals, "==".to_string()))
            } else {
                Some(gen_token(TokenType::Equals, "=".to_string())) // 'single equals'
            }
        },
        '!' => {
            if let Some(&'=') = src.peek() {
                src.next();
                Some(gen_token(TokenType::NotEquals, "!=".to_string()))
            } else {
                panic!("Unrecognized character: '!' (expected !=)");
            }
        },
        '<' => {
            if let Some(&'=') = src.peek() {
                src.next();
                Some(gen_token(TokenType::LessThanEquals, "<=".to_string()))
            } else {
                Some(gen_token(TokenType::LessThan, "<".to_string()))
            }
        },
        '>' => {
            if let Some(&'=') = src.peek() {
                src.next();
                Some(gen_token(TokenType::GreaterThanEquals, ">=".to_string()))
            } else {
                Some(gen_token(TokenType::GreaterThan, ">".to_string()))
            }
        },
        _ if is_digit(current_char) => {
            Some(gen_token(TokenType::Number, build_number(src, current_char)))
        }
        _ if is_alpha(current_char) => {
            let identifier_str = build_identifier(src, current_char);
            match identifier_str.as_str() {
                "null" => Some(gen_token(TokenType::Null, identifier_str)),
                "if" => Some(gen_token(TokenType::If, identifier_str)),
                "else" => Some(gen_token(TokenType::Else, identifier_str)),
                "until" => Some(gen_token(TokenType::Until, identifier_str)),
                "unless" => Some(gen_token(TokenType::Unless, identifier_str)),
                "while" => Some(gen_token(TokenType::While, identifier_str)),
                "work" => Some(gen_token(TokenType::Work, identifier_str)),
                "interop" => Some(gen_token(TokenType::Interop, identifier_str)),
                "return" => Some(gen_token(TokenType::Return, identifier_str)),
                "continue" => Some(gen_token(TokenType::Continue, identifier_str)),
                "break" => Some(gen_token(TokenType::Break, identifier_str)),
                "def" => Some(gen_token(TokenType::Def, identifier_str)),
                "fn" => Some(gen_token(TokenType::Function, identifier_str)),
                "async" => Some(gen_token(TokenType::Async, identifier_str)),
                "import" => Some(gen_token(TokenType::Import, identifier_str)),
                _ => Some(gen_token(TokenType::Identifier, identifier_str)),
            }
        }
        _ if is_eof(current_char) || is_whitespace(current_char) => {
            next_token(src)
        }
        _ => panic!("Unrecognized character: '{}'", current_char),
    }
}

fn consume_while<F>(src: &mut Peekable<Chars>, test: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();
    while let Some(&c) = src.peek() {
        if test(c) {
            result.push(src.next().unwrap());
        } else {
            break;
        }
    }
    result
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut src: Peekable<Chars> = source_code.chars().peekable();

    let mut tokens = Vec::new();

    while let Some(token) = next_token(&mut src) {
        tokens.push(token);
    }

    tokens
}