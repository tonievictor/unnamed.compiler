#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Constant,
    Keyword(String),
    OParen,
    CParen,
    OBrace,
    CBrace,
    OAngle,
    CAngle,
    SemiColon,
    Plus,
    Minus,
    Divide,
    Multiply,
    Modulo,
    Illegal,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token_literal: String,
    pub line_num: u32,
    pub col_num: u32,
}

pub fn tokenize(file_content: String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;
    let mut col = 0;
    let mut chars = file_content.chars().peekable();
    loop {
        let token: Token;
        match chars.next() {
            Some(c) => {
                col = col + 1;
                if c == '\n' {
                    line =  line + 1;
                    col = 0;
                    continue;
                }
                match c {
                    '(' => { token = create_token(TokenType::OParen, String::from(c), line, col); }
                    ')' => { token = create_token(TokenType::CParen, String::from(c), line, col); }
                    '<' => { token = create_token(TokenType::OAngle, String::from(c), line, col); }
                    '>' => { token = create_token(TokenType::CAngle, String::from(c), line, col); }
                    '{' => { token = create_token(TokenType::OBrace, String::from(c), line, col); }
                    '}' => { token = create_token(TokenType::CBrace, String::from(c), line, col); }
                    '+' => { token = create_token(TokenType::Plus, String::from(c), line, col); }
                    '-' => { token = create_token(TokenType::Minus, String::from(c), line, col); }
                    '/' => { token = create_token(TokenType::Divide, String::from(c), line, col); }
                    '*' => { token = create_token(TokenType::Multiply, String::from(c), line, col); }
                    '%' => { token = create_token(TokenType::Modulo, String::from(c), line, col); }
                    ';' => { token = create_token(TokenType::SemiColon, String::from(c), line, col); }
                    ' ' | '\t' => {
                        continue;
                    }
                    c => {
                        let mut i = 0;
                        if c.is_ascii_alphabetic() {
                            let mut tok = String::from(c);
                            while let Some(t) = chars.next_if(|&x| x.is_ascii_alphabetic()) {
                                i = i + 1;
                                tok.push(t);
                            }
                            token = create_token(TokenType::Identifier, tok, line, col);
                            col = col + i;
                        } else if c.is_ascii_digit() {
                            i = i + 1;
                            let mut tok = String::from(c);
                            while let Some(t) = chars.next_if(|&x| x.is_ascii_digit()) {
                                tok.push(t);
                            }
                            token = create_token(TokenType::Constant, tok, line, col);
                            col = col + i;
                        } else {
                            token = create_token(TokenType::Illegal, String::from(c), line, col)
                        }
                    }
                }
                tokens.push(token);
            }
            None => { break; }
        }
    }
    if tokens.is_empty() {
        return None;
    }
    return Some(tokens);
}

fn create_token(tok_type: TokenType, literal: String, line: u32, col: u32) -> Token {
    return Token {
        token_type: tok_type,
        token_literal: literal,
        line_num: line,
        col_num: col,
    };
}
