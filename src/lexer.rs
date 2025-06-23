#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeywordType {
    Int,
    Return,
    Pub,
    Fn,
}

#[derive(PartialEq, Clone, Debug)]
pub enum NumberType {
    Int(u32),
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Identifier,
    Number(NumberType),
    String(String),
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
    WhiteSpace,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
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
                tokens.push(create_token(TokenType::OParen, lnum, cnum));
            }
            Some(')') => {
                tokens.push(create_token(TokenType::CParen, lnum, cnum));
            }
            Some('{') => {
                tokens.push(create_token(TokenType::OBrace, lnum, cnum));
            }
            Some('}') => {
                tokens.push(create_token(TokenType::CBrace, lnum, cnum));
            }
            Some('+') => {
                tokens.push(create_token(TokenType::Plus, lnum, cnum));
            }
            Some('*') => {
                tokens.push(create_token(TokenType::Multiply, lnum, cnum));
            }
            Some(';') => {
                tokens.push(create_token(TokenType::SemiColon, lnum, cnum));
            }
            Some('=') => {
                tokens.push(create_token(TokenType::Equal, lnum, cnum));
            }
            Some(' ') | Some('\t') => {
                tokens.push(create_token(TokenType::WhiteSpace, lnum, cnum));
                while let Some(_) = chars.next_if(|&x| matches!(x, ' ' | '\t')) {
                    cnum += 1;
                }
            }
            Some('-') => {
                let t = match chars.peek() {
                    Some('>') => {
                        chars.next();
                        let coln = cnum;
                        cnum += 1;
                        create_token(TokenType::Arrow, lnum, coln)
                    }
                    _ => create_token(TokenType::Minus, lnum, cnum),
                };
                tokens.push(t);
            }
            Some('/') => {
                if let Some(next_char) = chars.peek() {
                    if *next_char == '/' {
                        while chars.next_if(|&x| x != '\n').is_some() {}
                        continue;
                    }
                }
                tokens.push(create_token(TokenType::Divide, lnum, cnum));
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
                tokens.push(create_token(TokenType::String(tok), lnum, i));
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

                    let num_literal = match tok.as_str().trim().parse::<u32>() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(format!(
                                "TYPE ERROR {{{}, {}}}, only integers are supported for now", lnum, cnum
                            ))
                        }
                    };
                    tokens.push(create_token(
                        TokenType::Number(NumberType::Int(num_literal)),
                        lnum,
                        cnum,
                    ));
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
        "int" => TokenType::Keyword(KeywordType::Int),
        "return" => TokenType::Keyword(KeywordType::Return),
        "pub" => TokenType::Keyword(KeywordType::Pub),
        "fn" => TokenType::Keyword(KeywordType::Fn),
        _ => TokenType::Identifier,
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

#[test]
fn test_string_tokenization() {
    let data = &[
        String::from(r#""Hello""#),
        String::from(r#""string\twith""#),
        String::from(r#""escaped \\\"quote\\\"""#),
        String::from(r#""multi\nline\nstring""#),
        String::from(r#""""#),
    ];

    for elem in data.iter() {
        let expected = vec![Token {
            ttype: TokenType::String(elem.clone()),
            line: 1,
            col: 1,
        }];
        assert_eq!(tokenize(elem.clone()), Ok(expected));
    }
}

#[test]
fn test_identifier_tokenization() {
    let data = &[
        String::from("Hello"),
        String::from("main_one"),
        String::from("main_2"),
    ];
    for elem in data.iter() {
        let expected = vec![Token {
            ttype: TokenType::Identifier,
            line: 1,
            col: 1,
        }];
        assert_eq!(tokenize(elem.clone()), Ok(expected));
    }
}

// #[test]
// fn test_keyword_tokenization() {
//     let data = &[
//         String::from("fn"),
//         String::from("return"),
//         String::from("pub"),
//         String::from("int"),
//     ];
//     for elem in data.iter() {
//         let expected = vec![Token {
//             token_type: TokenType::Keyword(),
//             token_value: elem.clone(),
//             line_num: 1,
//             col_num: 1,
//         }];
//         assert_eq!(tokenize(elem.clone()), Ok(expected));
//     }
// }
