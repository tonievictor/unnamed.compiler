#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeywordType {
    Fn,
    Return,
    Let,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType<'a> {
    Constant(&'a str),
    Identifier(&'a str),
    StringLiteral(&'a str),
    Keyword(KeywordType),
    OParen,
    CParen,
    OBrace,
    CBrace,
    Semicolon,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub ttype: TokenType<'a>,
    pub line: u32,
    pub col: u32,
}

pub fn tokenize<'a>(code: &'a str) -> Result<Vec<Token<'a>>, String> {
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
                tokens.push(create_token(TokenType::OParen, lnum, cnum));
            }
            Some((_, ')')) => {
                tokens.push(create_token(TokenType::CParen, lnum, cnum));
            }
            Some((_, '{')) => {
                tokens.push(create_token(TokenType::OBrace, lnum, cnum));
            }
            Some((_, '}')) => {
                tokens.push(create_token(TokenType::CBrace, lnum, cnum));
            }
            Some((_, ';')) => {
                tokens.push(create_token(TokenType::Semicolon, lnum, cnum));
            }
            Some((_, '=')) => {
                tokens.push(create_token(TokenType::Equal, lnum, cnum));
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
                    TokenType::StringLiteral(&code[start + 1..end as usize]),
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
                    TokenType::Constant(&code[start..start + (cnum - i) as usize + 1]),
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
        "fn" => TokenType::Keyword(KeywordType::Fn),
        "let" => TokenType::Keyword(KeywordType::Let),
        "return" => TokenType::Keyword(KeywordType::Return),
        _ => TokenType::Identifier(tok),
    };

    create_token(ttype, line, col)
}

fn create_token(tok_type: TokenType, line: u32, col: u32) -> Token {
    Token {
        ttype: tok_type,
        line,
        col,
    }
}
