use crate::lexer::{KeywordType, Token, TokenType};
use std::slice::Iter;

pub fn parse(tokens: &mut Iter<Token>) {
    while let Some(tok) = tokens.next() {
        match tok.ttype {
            TokenType::Keyword(KeywordType::Let) => {
                match parse_let_statement(tok.clone(), tokens) {
                    Ok(ltstmt) => println!("{ltstmt:#?}"),
                    Err(err) => println!("{err}"),
                }
            }
            TokenType::Keyword(KeywordType::Fn) => {}
            _ => {}
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Expression {
    Identifier(String),
    // StringLiteral(String),
    IntegerLiteral(u32),
}

#[derive(PartialEq, Clone, Debug)]
struct LetStatement {
    token: Token,
    name: Expression,
    value: Expression,
}

fn parse_let_statement(
    curr_token: Token,
    tokens: &mut Iter<Token>,
) -> Result<LetStatement, String> {
    let name = get_and_expect(tokens, TokenType::Identifier)?;
    get_and_expect(tokens, TokenType::Equal)?;
    let value = parse_expression(tokens)?;
    get_and_expect(tokens, TokenType::Semicolon)?;

    Ok(LetStatement {
        token: curr_token,
        name: Expression::Identifier(name.literal),
        value,
    })
}

fn parse_expression(tokens: &mut Iter<Token>) -> Result<Expression, String> {
    let curr_token = match tokens.next() {
        Some(t) => t,
        None => return Err("Expected an expression , but received nothing (None)".to_string()),
    };

    match curr_token.ttype {
        TokenType::Constant => parse_integer_expression(curr_token.clone(), tokens),
        _ => todo!("handle more cases"),
    }
}

fn parse_integer_expression(
    curr_token: Token,
    tokens: &mut Iter<Token>,
) -> Result<Expression, String> {
    match tokens.clone().peekable().peek() {
        Some(t) => match t.ttype {
            TokenType::Semicolon => {
                let num = curr_token
                    .literal
                    .parse::<u32>()
                    .expect("this should always be a valid number");
                Ok(Expression::IntegerLiteral(num))
            }
            _ => todo!("handle more cases"),
        },
        None => Err("Expected an expression, but received nothing (None)".to_string()),
    }
}

macro_rules! expect_err {
    ($line:expr, $col:expr, $expect:expr, $got:expr) => {
        format!(
            "ERROR {{{},{}}}: expected {:?}, got {:?}",
            $line, $col, $expect, $got
        )
    };
}

fn get_and_expect(tokens: &mut Iter<Token>, expect: TokenType) -> Result<Token, String> {
    match tokens.next() {
        Some(tok) => {
            if tok.ttype == expect {
                return Ok(tok.clone());
            }
            Err(expect_err!(tok.line, tok.col, expect, tok.ttype))
        }
        None => Err(format!(
            "Expected value {expect:?}, but received nothing (None)",
        )),
    }
}
