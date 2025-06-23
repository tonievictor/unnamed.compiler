#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeywordType {
    Int,
    Return,
    Pub,
    Fn,
}

#[derive(PartialEq, Clone, Debug)]
pub enum NumberType {
    Int(i32),
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line_num: u32,
    pub col_num: u32,
}

pub fn tokenize(code: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;
    let mut col = 0;
    let mut chars = code.chars().peekable();
    loop {
        col += 1;
        match chars.next() {
            Some('\n') => {
                line += 1;
                col = 0;
                continue;
            }
            Some('(') => {
                tokens.push(create_token(TokenType::OParen, line, col));
            }
            Some(')') => {
                tokens.push(create_token(TokenType::CParen, line, col));
            }
            Some('{') => {
                tokens.push(create_token(TokenType::OBrace, line, col));
            }
            Some('}') => {
                tokens.push(create_token(TokenType::CBrace, line, col));
            }
            Some('+') => {
                tokens.push(create_token(TokenType::Plus, line, col));
            }
            Some('*') => {
                tokens.push(create_token(TokenType::Multiply, line, col));
            }
            Some(';') => {
                tokens.push(create_token(TokenType::SemiColon, line, col));
            }
            Some('=') => {
                tokens.push(create_token(TokenType::Equal, line, col));
            }
            Some(' ') | Some('\t') => {
                continue;
            }
            Some('-') => {
                let t = match chars.peek() {
                    Some('>') => {
                        chars.next();
                        let coln = col;
                        col += 1;
                        create_token(TokenType::Arrow, line, coln)
                    }
                    _ => create_token(TokenType::Minus, line, col),
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
                tokens.push(create_token(TokenType::Divide, line, col));
            }
            Some('"') => {
                let i = col;
                let mut tok = String::from("\"");
                loop {
                    col += 1;
                    match chars.next() {
                        Some('"') => {
                            tok.push('"');
                            break;
                        },
                        Some('\\') => {
                            match chars.next() {
                                Some(c) => {
                                    col +=1;
                                    tok.push('\\');
                                    tok.push(c);
                                }
                                None => return Err(format!("SYNTAX ERROR {},{}: unexpected end of file found before string literal termination", line, col))
                            }
                        }
                        Some('\n') => return Err(format!("SYNTAX ERROR {}, {}: multiline Strings are not supported", line, col)),
                        Some(c) => tok.push(c),
                        None => return Err(format!("SYNTAX ERROR {},{}: unexpected end of file found before string literal termination", line, col))
                    }
                }
                tokens.push(create_token(TokenType::String(tok), line, i));
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
                    tokens.push(create_keyword_or_identifier(tok, line, col));

                    col += i;
                } else if c.is_ascii_digit() {
                    let mut tok = String::from(c);
                    while let Some(t) = chars.next_if(|&x| x.is_ascii_digit()) {
                        i += 1;
                        tok.push(t);
                    }

                    // if let Some(next_char) = chars.peek() {
                    //     if !matches!(next_char, ' ' | ';') {
                    //         return Err(format!(
                    //             "{}:{}: invalid suffix on integer constant",
                    //             line, col
                    //         ));
                    //     }
                    // }

                    let num_literal = match tok.as_str().trim().parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(format!(
                                "Invalid Number Literal, only integers are supported for now"
                            ))
                        }
                    };
                    tokens.push(create_token(
                        TokenType::Number(NumberType::Int(num_literal)),
                        line,
                        col,
                    ));
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
        token_type: tok_type,
        line_num: line,
        col_num: col,
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
            token_type: TokenType::String(elem.clone()),
            line_num: 1,
            col_num: 1,
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
            token_type: TokenType::Identifier,
            line_num: 1,
            col_num: 1,
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
