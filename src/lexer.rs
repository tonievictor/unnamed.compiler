#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeywordType {
    Int,
    Return,
    Pub,
    Fn,
    // pub fn main() -> int {
    // 	return 2;
    // }
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
                        if c == '-' {
                            token = match chars.peek() {
                                Some(next_char) => {
                                    if *next_char == '>' {
                                        chars.next();
                                        let coln = col;
                                        col += 1;
                                        create_token(
                                            TokenType::Arrow,
                                            String::from("->"),
                                            line,
                                            coln,
                                        )
                                    } else {
                                        create_token(TokenType::Minus, String::from(c), line, col)
                                    }
                                }
                                None => create_token(TokenType::Minus, String::from(c), line, col),
                            }
                        } else if c == '/' {
                            if let Some(next_char) = chars.peek() {
                                if *next_char == '/' {
                                    while chars.next_if(|&x| x != '\n').is_some() {}
                                    continue;
                                }
                            }
                            token = create_token(TokenType::Divide, String::from(c), line, col);
                        } else if c.is_ascii_alphabetic() {
                            let mut tok = String::from(c);
                            while let Some(t) = chars
                                .next_if(|&x| matches!(x, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
                            {
                                i += 1;
                                tok.push(t);
                            }
                            token = create_keyword_or_identifier(tok, line, col);

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
                            token = create_token(TokenType::Number, tok, line, col);
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
