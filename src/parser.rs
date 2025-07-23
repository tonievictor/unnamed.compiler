use crate::lexer::{Keyword, Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn parse(tokens: Vec<Token>) -> Result<(), String> {
    let mut tokens = tokens.into_iter().peekable();

    while let Some(tok) = tokens.next() {
        match tok.kind {
            TokenKind::Keyword(Keyword::Let) => match parse_let_stmt(&mut tokens) {
                Ok(stmt) => println!("{stmt:?}"),
                Err(err) => {
                    eprintln!("{err}");
                    break;
                }
            },
            _ => {
                println!("others");
                return Ok(());
            }
        }
    }
    Ok(())
}

pub trait Statement {
    fn statement(&self) {
        println!("this is a statement");
    }
}

macro_rules! expect_error {
    ($expected:expr, $tok:expr) => {
        Err(format!(
            "Expected {} at line {}, column {}, but got {:?}",
            $expected, $tok.line, $tok.col, $tok.kind
        ))
    };
}

macro_rules! unexpected_error {
    ($kind:expr,$tok:expr) => {
        Err(format!(
            "Unexpected token after {} at line {}, column {}: {:?}",
            $kind, $tok.line, $tok.col, $tok.kind
        ))
    };
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LetStmt<'a> {
    pub kind: Keyword,
    pub name: &'a str,
    value: Expression<'a>,
}

pub fn parse_let_stmt<'a>(
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<LetStmt<'a>, String> {
    let name = expect_identifier(tokens)?;
    expect_or_err(tokens, TokenKind::Equal)?;
    let value = parse_expr(tokens)?;
    expect_or_err(tokens, TokenKind::Semicolon)?;

    Ok(LetStmt {
        kind: Keyword::Let,
        name,
        value,
    })
}

pub fn parse_expr<'a>(
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<Expression<'a>, String> {
    match tokens.next() {
        Some(tok) => match tok.kind {
            TokenKind::Constant(val) => parse_int_expr(val, tokens),
            TokenKind::StringLiteral(val) => parse_string_expr(val, tokens),
            TokenKind::Identifier(val) => parse_iden_expr(val, tokens),
            kind => Err(format!(
                "Unsupported expression starting with token: {kind:?}",
            )),
        },
        None => Err("Unexpected end of input while expecting an expression.".to_string()),
    }
}

pub fn parse_string_expr<'a>(
    val: &'a str,
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<Expression<'a>, String> {
    match tokens.peek() {
        Some(tok) if tok.kind == TokenKind::Semicolon => Ok(Expression::StringLiteral(val)),
        Some(tok) => unexpected_error!("string literal", tok),
        None => Err("Unexpected end of input while expecting an expression.".to_string()),
    }
}

pub fn parse_iden_expr<'a>(
    val: &'a str,
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<Expression<'a>, String> {
    match tokens.peek() {
        Some(tok) if tok.kind == TokenKind::Semicolon => Ok(Expression::IdentifierLiteral(val)),
        Some(tok) => unexpected_error!("identifier", tok),
        None => Err("Unexpected end of input while expecting an expression.".to_string()),
    }
}

pub fn parse_int_expr<'a>(
    val: &'a str,
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<Expression<'a>, String> {
    match tokens.peek() {
        Some(tok) if tok.kind == TokenKind::Semicolon => match val.parse::<u32>() {
            Ok(val) => Ok(Expression::IntegerLiteral(val)),
            Err(err) => Err(format!(
                "Error while parsing integer literal at line {}, column {}: {}",
                tok.line, tok.col, err
            )),
        },
        Some(tok) => unexpected_error!("integer", tok),
        None => Err("Unexpected end of input while expecting an expression.".to_string()),
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    IntegerLiteral(u32),
    StringLiteral(&'a str),
    IdentifierLiteral(&'a str),
}

pub fn expect_identifier<'a>(
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<&'a str, String> {
    match tokens.next() {
        Some(tok) => match tok.kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => expect_error!("identifier", tok),
        },
        None => Err("Unexpected end of input while expecting a token.".to_string()),
    }
}
pub fn expect_or_err<'a>(
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
    kind: TokenKind<'a>,
) -> Result<(), String> {
    match tokens.next() {
        Some(tok) if tok.kind != kind => expect_error!(kind, tok),
        Some(_) => Ok(()),
        None => Err("Unexpected end of input while expecting a token.".to_string()),
    }
}
