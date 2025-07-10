#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeywordType {
    Fn,
    Return,
    Let,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Constant,
    Identifier,
    StringLiteral,
    Keyword(KeywordType),
    OParen,
    CParen,
    OBrace,
    CBrace,
    Semicolon,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
    pub line: u32,
    pub col: u32,
}

pub fn tokenize(code: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lnum = 1;
    let mut cnum = 0;
    let mut chars = code.chars().peekable();
    loop {
        cnum += 1;
        match chars.next() {
            Some('\n') => {
                lnum += 1;
                cnum = 0;
                continue;
            }
            Some('(') => {
                tokens.push(create_token(
                    TokenType::OParen,
                    String::from("("),
                    lnum,
                    cnum,
                ));
            }
            Some(')') => {
                tokens.push(create_token(
                    TokenType::CParen,
                    String::from(")"),
                    lnum,
                    cnum,
                ));
            }
            Some('{') => {
                tokens.push(create_token(
                    TokenType::OBrace,
                    String::from("{"),
                    lnum,
                    cnum,
                ));
            }
            Some('}') => {
                tokens.push(create_token(
                    TokenType::CBrace,
                    String::from("}"),
                    lnum,
                    cnum,
                ));
            }
            Some(';') => {
                tokens.push(create_token(
                    TokenType::Semicolon,
                    String::from(";"),
                    lnum,
                    cnum,
                ));
            }
            Some('=') => {
                tokens.push(create_token(
                    TokenType::Equal,
                    String::from("="),
                    lnum,
                    cnum,
                ));
            }
            Some(' ') | Some('\t') => {
                continue;
            }
            Some('/') => {
                if let Some(next_char) = chars.peek() {
                    if *next_char == '/' {
                        while chars.next_if(|&x| x != '\n').is_some() {}
                        continue;
                    }
                }
                return Err(format!(
                    "{}:{}: Illegal character '{}' in program",
                    lnum, cnum, '/'
                ));
            }
            Some('"') => {
                let i = cnum;
                let mut tok = String::from("\"");
                loop {
                    cnum += 1;
                    match chars.next() {
                        Some('"') => {
                            tok.push('"');
                            break;
                        },
                        Some('\\') => {
                            match chars.next() {
                                Some(c) => {
                                    cnum +=1;
                                    tok.push('\\');
                                    tok.push(c);
                                }
                                None => return Err(format!("SYNTAX ERROR {{{},{}}}: unexpected end of file found before string literal termination", lnum, cnum))
                            }
                        }
                        Some('\n') => return Err(format!("SYNTAX ERROR {{{},{}}}: unexpected end of file before string literal termination", lnum, cnum)),
                        Some(c) => tok.push(c),
                        None => return Err(format!("SYNTAX ERROR {{{},{}}}: unexpected end of file found before string literal termination", lnum, cnum))
                    }
                }
                tokens.push(create_token(TokenType::StringLiteral, tok, lnum, i));
            }
            Some(c) => {
                let mut i = 0;
                if c.is_ascii_alphabetic() {
                    let mut tok = String::from(c);
                    while let Some(t) =
                        chars.next_if(|&x| matches!(x, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
                    {
                        i += 1;
                        tok.push(t);
                    }
                    tokens.push(create_keyword_or_identifier(tok, lnum, cnum));

                    cnum += i;
                } else if c.is_ascii_digit() {
                    let mut tok = String::from(c);
                    while let Some(t) = chars.next_if(|&x| x.is_ascii_digit()) {
                        i += 1;
                        tok.push(t);
                    }

                    tokens.push(create_token(TokenType::Constant, tok, lnum, cnum));
                    cnum += i;
                } else {
                    return Err(format!(
                        "{}:{}: Illegal character '{}' in program",
                        lnum, cnum, c
                    ));
                }
            }
            None => {
                break;
            }
        }
    }

    Ok(tokens)
}

fn create_keyword_or_identifier(tok: String, line: u32, col: u32) -> Token {
    let ttype = match tok.as_str() {
        "fn" => TokenType::Keyword(KeywordType::Fn),
        "let" => TokenType::Keyword(KeywordType::Let),
        "return" => TokenType::Keyword(KeywordType::Return),
        _ => TokenType::Identifier,
    };

    create_token(ttype, tok, line, col)
}

fn create_token(tok_type: TokenType, literal: String, line: u32, col: u32) -> Token {
    Token {
        ttype: tok_type,
        literal,
        line,
        col,
    }
}
