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
            TokenKind::Keyword(Keyword::Return) => match parse_ret_stmt(&mut tokens) {
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

fn parse_ret_stmt<'a>(
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<ReturnStmt<'a>, String> {
    expect_or_err(tokens, TokenKind::OParen)?;
    let value = parse_expr(tokens, TokenKind::CParen)?;
    expect_or_err(tokens, TokenKind::CParen)?;
    expect_or_err(tokens, TokenKind::Semicolon)?;
    Ok(ReturnStmt { value })
}

#[derive(Debug)]
#[allow(dead_code)]
struct ReturnStmt<'a> {
    value: Expression<'a>,
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
struct LetStmt<'a> {
    kind: Keyword,
    name: &'a str,
    value: Expression<'a>,
}

fn parse_let_stmt<'a>(tokens: &mut Peekable<IntoIter<Token<'a>>>) -> Result<LetStmt<'a>, String> {
    let name = expect_identifier(tokens)?;
    expect_or_err(tokens, TokenKind::Equal)?;
    let value = parse_expr(tokens, TokenKind::Semicolon)?;
    expect_or_err(tokens, TokenKind::Semicolon)?;

    Ok(LetStmt {
        kind: Keyword::Let,
        name,
        value,
    })
}

fn parse_expr<'a>(
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
    delimiter: TokenKind,
) -> Result<Expression<'a>, String> {
    match tokens.next() {
        Some(tok) => match tok.kind {
            TokenKind::Constant(val) => parse_int_expr(val, delimiter, tokens),
            TokenKind::StringLiteral(val) => parse_string_expr(val, delimiter, tokens),
            TokenKind::Identifier(val) => parse_iden_expr(val, delimiter, tokens),
            kind => Err(format!(
                "Unsupported expression starting with token: {kind:?}",
            )),
        },
        None => Err("Unexpected end of input while expecting an expression.".to_string()),
    }
}

fn parse_string_expr<'a>(
    val: &'a str,
    delimiter: TokenKind,
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<Expression<'a>, String> {
    match tokens.peek() {
        Some(tok) if tok.kind == delimiter => Ok(Expression::StringLiteral(val)),
        Some(tok) => unexpected_error!("string literal", tok),
        None => Err("Unexpected end of input while expecting an expression.".to_string()),
    }
}

fn parse_iden_expr<'a>(
    val: &'a str,
    delimiter: TokenKind,
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<Expression<'a>, String> {
    match tokens.peek() {
        Some(tok) if tok.kind == delimiter => Ok(Expression::IdentifierLiteral(val)),
        Some(tok) => unexpected_error!("identifier", tok),
        None => Err("Unexpected end of input while expecting an expression.".to_string()),
    }
}

fn parse_int_expr<'a>(
    val: &'a str,
    delimiter: TokenKind,
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
) -> Result<Expression<'a>, String> {
    match tokens.peek() {
        Some(tok) if tok.kind == delimiter => match val.parse::<u32>() {
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

fn expect_identifier<'a>(tokens: &mut Peekable<IntoIter<Token<'a>>>) -> Result<&'a str, String> {
    match tokens.next() {
        Some(tok) => match tok.kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => expect_error!("identifier", tok),
        },
        None => Err("Unexpected end of input while expecting a token.".to_string()),
    }
}

fn expect_or_err<'a>(
    tokens: &mut Peekable<IntoIter<Token<'a>>>,
    kind: TokenKind<'a>,
) -> Result<(), String> {
    match tokens.next() {
        Some(tok) if tok.kind != kind => expect_error!(kind, tok),
        Some(_) => Ok(()),
        None => Err("Unexpected end of input while expecting a token.".to_string()),
    }
}
