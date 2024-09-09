#[derive(Debug)]
enum TokenType {
    Identifier,
    Constant,
    Keyword,
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
    Symbol(String),
    Illegal,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token_literal: String,
    pub line_num: u32,
    pub col_num: u32,
}


pub fn tokenize(file: String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;
    let mut col = 0;
    let mut chars = file.chars().peekable();
    loop {
        let token: Token;
        match chars.next() {
            Some(c) => {
                if c == '\n' {
                    line =  line + 1;
                    col = 0;
                    continue;
                }
                match c {
                    '(' => { token = createToken(TokenType::OParen, String::from(c), line, col); }
                    ')' => { token = createToken(TokenType::CParen, String::from(c), line, col); }
                    '<' => { token = createToken(TokenType::OAngle, String::from(c), line, col); }
                    '>' => { token = createToken(TokenType::CAngle, String::from(c), line, col); }
                    '{' => { token = createToken(TokenType::OBrace, String::from(c), line, col); }
                    '}' => { token = createToken(TokenType::CBrace, String::from(c), line, col); }
                    '+' => { token = createToken(TokenType::Plus, String::from(c), line, col); }
                    '-' => { token = createToken(TokenType::Minus, String::from(c), line, col); }
                    '/' => { token = createToken(TokenType::Divide, String::from(c), line, col); }
                    '*' => { token = createToken(TokenType::Multiply, String::from(c), line, col); }
                    '%' => { token = createToken(TokenType::Modulo, String::from(c), line, col); }
                    ';' => { token = createToken(TokenType::SemiColon, String::from(c), line, col); }
                    ' ' | '\t' => {
                        continue;
                    }
                    c => {
                        if is_letter(c as u32) {
                            let mut tok = String::from(c);
                            while let Some(t) = chars.next_if(|&x| is_letter(x as u32)) {
                                tok.push(t);
                            }
                            token = createToken(TokenType::Identifier, tok, line, col);
                        } else if is_digit(c as u8) {
                            let mut tok = String::from(c);
                            while let Some(t) = chars.next_if(|&x| is_digit(x as u8)) {
                                tok.push(t);
                            }
                            token = createToken(TokenType::Constant, tok, line, col);
                        } else {
                            token = createToken(TokenType::Illegal, String::from(c), line, col)
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

fn is_letter(c: u32) -> bool {
    return (c >= 65 && c <= 90) || (c >= 97 && c <= 122);
}

fn createToken(tok_type: TokenType, literal: String, line: u32, col: u32) -> Token {
    return Token {
        token_type: tok_type,
        token_literal: literal,
        line_num: line,
        col_num: col,
    };
}
fn is_digit(c: u8) -> bool {
    return c >= 0 && c <= 9;
}
