use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keyword {
    Fn,
    Return,
    Let,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum TokenKind<'a> {
    Constant(&'a str),
    Identifier(&'a str),
    StringLiteral(&'a str),
    Keyword(Keyword),
    OParen,
    CParen,
    OBrace,
    CBrace,
    Semicolon,
    Equal,
}

impl<'a> Display for TokenKind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TokenKind::OParen => write!(f, "{}", '('),
            TokenKind::CParen => write!(f, "{}", ')'),
            TokenKind::CBrace => write!(f, "{}", '}'),
            TokenKind::OBrace => write!(f, "{}", '{'),
            TokenKind::Semicolon => write!(f, "{}", ';'),
            TokenKind::Equal => write!(f, "{}", '='),
            TokenKind::Keyword(Keyword::Fn) => write!(f, "fn"),
            TokenKind::Keyword(Keyword::Return) => write!(f, "return"),
            TokenKind::Keyword(Keyword::Let) => write!(f, "let"),
            TokenKind::Constant(constant) => write!(f, "{}", constant),
            TokenKind::Identifier(iden) => write!(f, "{}", iden),
            TokenKind::StringLiteral(literal) => write!(f, "{}", literal),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line: u32,
    pub col: u32,
}

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lnum = 1;
    let mut cnum = 0;
    let mut chars = code.char_indices().peekable();
    loop {
        cnum += 1;
        match chars.next() {
            Some((_, '\n')) => {
                lnum += 1;
                cnum = 0;
                continue;
            }
            Some((_, '(')) => {
                tokens.push(create_token(TokenKind::OParen, lnum, cnum));
            }
            Some((_, ')')) => {
                tokens.push(create_token(TokenKind::CParen, lnum, cnum));
            }
            Some((_, '{')) => {
                tokens.push(create_token(TokenKind::OBrace, lnum, cnum));
            }
            Some((_, '}')) => {
                tokens.push(create_token(TokenKind::CBrace, lnum, cnum));
            }
            Some((_, ';')) => {
                tokens.push(create_token(TokenKind::Semicolon, lnum, cnum));
            }
            Some((_, '=')) => {
                tokens.push(create_token(TokenKind::Equal, lnum, cnum));
            }
            Some((_, ' ')) | Some((_, '\t')) => {
                continue;
            }
            Some((_, '/')) => {
                if let Some((_, next_char)) = chars.peek() {
                    if *next_char == '/' {
                        while chars.next_if(|&(_, x)| x != '\n').is_some() {}
                        continue;
                    }
                }
                return Err(format!(
                    "{}:{}: Illegal character '{}' in program",
                    lnum, cnum, '/'
                ));
            }
            Some((start, '"')) => {
                let i = cnum;
                let end = loop {
                    cnum += 1;
                    match chars.next() {
                        Some((e, '"')) => break e,
                        Some((_, '\n')) => return Err(
                            format!("SYNTAX ERROR {lnum},{cnum}: unexpected end of file before string literal termination"
                        )),
                        Some(_) => continue,
                        None => return Err(
                            format!("SYNTAX ERROR {lnum},{cnum}: unexpected end of file found before string literal termination"
                        ))
                    }
                };
                tokens.push(create_token(
                    TokenKind::StringLiteral(&code[start + 1..end as usize]),
                    lnum,
                    i,
                ));
            }
            Some((start, 'a'..='z' | 'A'..='Z' | '_')) => {
                let i = cnum;
                while chars
                    .next_if(|&(_, x)| matches!(x, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
                    .is_some()
                {
                    cnum += 1;
                }
                tokens.push(create_keyword_or_identifier(
                    &code[start..start + (cnum - i) as usize + 1],
                    lnum,
                    i,
                ))
            }
            Some((start, '0'..='9')) => {
                let i = cnum;
                while chars.next_if(|&(_, x)| x.is_ascii_digit()).is_some() {
                    cnum += 1;
                }
                tokens.push(create_token(
                    TokenKind::Constant(&code[start..start + (cnum - i) as usize + 1]),
                    lnum,
                    i,
                ))
            }
            Some((_, c)) => {
                return Err(format!("{lnum}:{cnum}: Illegal character '{c}' in program"));
            }
            None => {
                break;
            }
        }
    }

    Ok(tokens)
}

fn create_keyword_or_identifier(tok: &str, line: u32, col: u32) -> Token {
    let ttype = match tok {
        "fn" => TokenKind::Keyword(Keyword::Fn),
        "let" => TokenKind::Keyword(Keyword::Let),
        "return" => TokenKind::Keyword(Keyword::Return),
        _ => TokenKind::Identifier(tok),
    };

    create_token(ttype, line, col)
}

fn create_token(tok_type: TokenKind, line: u32, col: u32) -> Token {
    Token {
        kind: tok_type,
        line,
        col,
    }
}
