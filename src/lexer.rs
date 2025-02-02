use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeywordType {
    Int,
    Return,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Identifier,
    Constant,
    Keyword(KeywordType),
    OParen,
    CParen,
    OBrace,
    CBrace,
    SemiColon,
    Plus,
    Minus,
    Divide,
    Equal,
    Multiply,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub line_num: u32,
    pub col_num: u32,
}

pub fn tokenize(file_content: String) -> Result<Option<Vec<Token>>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;
    let mut col = 0;
    let mut chars = file_content.chars().peekable();
    loop {
        let token: Token;
        match chars.next() {
            Some(c) => {
                col += 1;
                match c {
                    '\n' => {
                        line += 1;
                        col = 0;
                        continue;
                    }
                    '(' => {
                        token = create_token(TokenType::OParen, String::from(c), line, col);
                    }
                    ')' => {
                        token = create_token(TokenType::CParen, String::from(c), line, col);
                    }
                    '{' => {
                        token = create_token(TokenType::OBrace, String::from(c), line, col);
                    }
                    '}' => {
                        token = create_token(TokenType::CBrace, String::from(c), line, col);
                    }
                    '+' => {
                        token = create_token(TokenType::Plus, String::from(c), line, col);
                    }
                    '-' => {
                        token = create_token(TokenType::Minus, String::from(c), line, col);
                    }
                    '*' => {
                        token = create_token(TokenType::Multiply, String::from(c), line, col);
                    }
                    ';' => {
                        token = create_token(TokenType::SemiColon, String::from(c), line, col);
                    }
                    '=' => {
                        token = create_token(TokenType::Equal, String::from(c), line, col);
                    }
                    ' ' | '\t' => {
                        continue;
                    }
                    c => {
                        let mut i = 0;
                        if c == '/' {
                            if let Some(next_char) = chars.peek() {
                                if *next_char == '/' {
                                    while chars.next_if(|&x| x != '\n').is_some() {}
                                    continue;
                                }
                            }
                            token = create_token(TokenType::Divide, String::from(c), line, col);
                        } else if c.is_ascii_alphabetic() {
                            let mut tok = String::from(c);
                            while let Some(t) = chars.next_if(|&x| x.is_ascii_alphabetic()) {
                                i += 1;
                                tok.push(t);
                            }
                            let keyword = find_keyword(&tok);
                            match keyword {
                                Some(keyword_type) => {
                                    token = create_token(
                                        TokenType::Keyword(keyword_type),
                                        tok,
                                        line,
                                        col,
                                    );
                                }
                                None => {
                                    token = create_token(TokenType::Identifier, tok, line, col);
                                }
                            }
                            col += i;
                        } else if c.is_ascii_digit() {
                            let mut tok = String::from(c);
                            while let Some(t) = chars.next_if(|&x| x.is_ascii_digit()) {
                                i += 1;
                                tok.push(t);
                            }

                            if let Some(next_char) = chars.peek() {
                                if next_char.is_ascii_alphabetic() {
                                    return Err(format!(
                                        "{}:{}: invalid suffix on integer constant",
                                        line, col
                                    ));
                                }
                            }
                            token = create_token(TokenType::Constant, tok, line, col);
                            col += i;
                        } else {
                            return Err(format!(
                                "{}:{}: Illegal character '{}' in program",
                                line, col, c
                            ));
                        }
                    }
                }
                tokens.push(token);
            }
            None => {
                break;
            }
        }
    }

    if tokens.is_empty() {
        return Ok(None);
    }
    Ok(Some(tokens))
}

fn find_keyword(tok: &String) -> Option<KeywordType> {
    let keywords = HashMap::from([
        ("int".to_string(), KeywordType::Int),
        ("return".to_string(), KeywordType::Return),
    ]);
    keywords.get(tok).copied()
}

fn create_token(tok_type: TokenType, value: String, line: u32, col: u32) -> Token {
    Token {
        token_type: tok_type,
        token_value: value,
        line_num: line,
        col_num: col,
    }
}
