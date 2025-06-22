#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeywordType {
    Int,
    Return,
    Pub,
    Fn,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Identifier,
    Number,
    String,
    Keyword(KeywordType),
    OParen,
    CParen,
    OBrace,
    CBrace,
    SemiColon,
    Plus,
    Minus,
    Divide,
    Arrow,
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
        col += 1;
        match chars.next() {
            Some('\n') => {
                line += 1;
                col = 0;
                continue;
            }
            Some('(') => {
                tokens.push(create_token(TokenType::OParen, String::from('('), line, col));
            }
            Some(')') => {
                tokens.push(create_token(TokenType::CParen, String::from(')'), line, col));
            }
            Some('{') => {
                tokens.push(create_token(TokenType::OBrace, String::from('{'), line, col));
            }
            Some('}') => {
                tokens.push(create_token(TokenType::CBrace, String::from('}'), line, col));
            }
            Some('+') => {
                tokens.push(create_token(TokenType::Plus, String::from('+'), line, col));
            }
            Some('*') => {
                tokens.push(create_token(TokenType::Multiply, String::from('*'), line, col));
            }
            Some(';') => {
                tokens.push(create_token(TokenType::SemiColon, String::from(';'), line, col));
            }
            Some('=') => {
                tokens.push(create_token(TokenType::Equal, String::from('='), line, col));
            }
            Some(' ') | Some('\t') => {
                continue;
            }
            Some(c) => {
                let mut i = 0;
                if c == '-' {
                    let t = match chars.peek() {
                        Some('>') => {
                            chars.next();
                            let coln = col;
                            col += 1;
                            create_token(TokenType::Arrow, String::from("->"), line, coln)
                        }
                        _ => create_token(TokenType::Minus, String::from(c), line, col),
                    };
                    tokens.push(t);
                } else if c == '/' {
                    if let Some(next_char) = chars.peek() {
                        if *next_char == '/' {
                            while chars.next_if(|&x| x != '\n').is_some() {}
                            continue;
                        }
                    }
                    tokens.push(create_token(TokenType::Divide, String::from(c), line, col));
                } else if c.is_ascii_alphabetic() {
                    let mut tok = String::from(c);
                    while let Some(t) =
                        chars.next_if(|&x| matches!(x, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
                    {
                        i += 1;
                        tok.push(t);
                    }
                    tokens.push(create_keyword_or_identifier(tok, line, col));

                    col += i;
                } else if c.is_ascii_digit() {
                    let mut tok = String::from(c);
                    while let Some(t) = chars.next_if(|&x| x.is_ascii_digit()) {
                        i += 1;
                        tok.push(t);
                    }

                    if let Some(next_char) = chars.peek() {
                        if !matches!(next_char, ' ' | ';') {
                            return Err(format!(
                                "{}:{}: invalid suffix on integer constant",
                                line, col
                            ));
                        }
                    }
                    tokens.push(create_token(TokenType::Number, tok, line, col));
                    col += i;
                } else {
                    return Err(format!(
                        "{}:{}: Illegal character '{}' in program",
                        line, col, c
                    ));
                }
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

fn create_keyword_or_identifier(tok: String, line: u32, col: u32) -> Token {
    let ttype = match tok.as_str() {
        "int" => TokenType::Keyword(KeywordType::Int),
        "return" => TokenType::Keyword(KeywordType::Return),
        "pub" => TokenType::Keyword(KeywordType::Pub),
        "fn" => TokenType::Keyword(KeywordType::Fn),
        _ => TokenType::Identifier,
    };

    create_token(ttype, tok, line, col)
}

fn create_token(tok_type: TokenType, value: String, line: u32, col: u32) -> Token {
    Token {
        token_type: tok_type,
        token_value: value,
        line_num: line,
        col_num: col,
    }
}
