pub struct Lexer {}

#[derive(Debug, Clone)]
pub enum TokenType {
    Comma,
    OpenParen,
    CloseParen,
    Binary,
    Number,
    Identifier,
    Dot,
    Null
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String
}

impl Lexer {
    pub fn is_int(token: &str) -> bool {
        token.parse::<f64>().is_ok()
    }

    pub fn gen_token(token_type: TokenType, value: String) -> Token {
        return Token {
            kind: token_type,
            value
        }
    }

    pub fn is_skipable(token: &str) -> bool {
        return token == "" || token == " " || token == "\n" || token == "\t"
    }

    pub fn get_first_item<'a>(src: &Vec<&'a str>) -> &'a str {
        match src.get(0) {
            Some(result) => result,
            None => ""
        }
    }

    pub fn is_char(token: &str) -> bool {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(token)
    }

    pub fn build_number(src: &mut Vec<&str>) -> String {
        let mut number = src.remove(0).to_string();
        let mut first_token = Lexer::get_first_item(src);
        while src.len() > 0 && Lexer::is_int(first_token) {
            number += src.remove(0);
            first_token = Lexer::get_first_item(&src);
        }
        return number;
    }

    pub fn build_identifer(src: &mut Vec<&str>) -> String {
        let mut identifier = src.remove(0).to_string();
        let mut first_token = Lexer::get_first_item(src);
        while src.len() > 0 && (Lexer::is_char(first_token) || Lexer::is_int(first_token)) {
            identifier += src.remove(0);
            first_token = Lexer::get_first_item(&src);
        }
        return identifier;
    }

    pub fn tokenize<'a>(source_code: &'a str) -> Vec<Token> {
        let mut src: Vec<&str> = source_code.split("").collect::<Vec<&str>>();
        let mut tokens: Vec::<Token> = vec![];

        while src.len() > 0 {
            let fist_token = Lexer::get_first_item(&src);
            if fist_token == "*" || fist_token == "/" || fist_token == "-" || fist_token == "+" || fist_token == "%" {
                tokens.push(Lexer::gen_token(TokenType::Binary, src.remove(0).to_string()))
            } else if fist_token == "(" {
                tokens.push(Lexer::gen_token(TokenType::OpenParen, src.remove(0).to_string()))
            } else if fist_token == ")" {
                tokens.push(Lexer::gen_token(TokenType::CloseParen, src.remove(0).to_string()))
            } else if fist_token == ";" {
                tokens.push(Lexer::gen_token(TokenType::Comma, src.remove(0).to_string()))
            } else if fist_token == "." {
                tokens.push(Lexer::gen_token(TokenType::Dot, src.remove(0).to_string()))
            } else {
                if Lexer::is_int(fist_token) {
                    tokens.push(Lexer::gen_token(TokenType::Number, Lexer::build_number(&mut src)))
                } else if Lexer::is_skipable(fist_token) {
                    src.remove(0);
                } else if Lexer::is_char(fist_token) {
                    tokens.push(Lexer::gen_token(TokenType::Identifier, Lexer::build_identifer(&mut src)))
                }  else {
                    panic!("Unrecognized token")
                }
            }
        }

        return tokens
    }
}