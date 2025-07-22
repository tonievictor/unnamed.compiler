jse crate::lexer::{Keyword, Token, TokenKind};
use std::slice::Iter;

pub fn parse(tokens: Vec<Token>) -> Result<(), String> {
    let mut tokens = tokens.iter();

    while let Some(tok) = tokens.next() {
        match tok.kind {
            TokenKind::Keyword(Keyword::Let) => {
                let stmt = parse_let_stmt(&mut tokens)?;
                println!("{stmt:?}");
            }
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
    ($expected:expr,$got:expr) => {
        Err(format!("expected {} but got {}", $expected, $got))
    };
}

pub fn parse_let_stmt<'a>(tokens: &mut Iter<'_, Token<'a>>) -> Result<LetStmt<'a>, String> {
    let name = expect_identifier(tokens)?;
    expect_or_err(tokens, TokenKind::Equal)?;
    // parse expressions
    expect_or_err(tokens, TokenKind::Semicolon)?;

    Ok(LetStmt {
        kind: Keyword::Let,
        name,
    })
}

pub fn expect_identifier<'a>(tokens: &mut Iter<'_, Token<'a>>) -> Result<&'a str, String> {
    match tokens.next() {
        Some(tok) => match tok.kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => expect_error!(TokenKind::Identifier(""), tok.kind),
        },
        None => expect_error!(TokenKind::Identifier(""), "none"),
    }
}
pub fn expect_or_err<'a>(
    tokens: &mut Iter<'_, Token<'a>>,
    kind: TokenKind<'a>,
) -> Result<(), String> {
    match tokens.next() {
        Some(tok) if tok.kind != kind => expect_error!(TokenKind::Identifier(""), tok.kind),
        Some(_) => Ok(()),
        None => expect_error!(TokenKind::Identifier(""), "none"),
    }
}

pub enum Expression {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LetStmt<'a> {
    pub kind: Keyword,
    pub name: &'a str,
    // value: Expression,
}

