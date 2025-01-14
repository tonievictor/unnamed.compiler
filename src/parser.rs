use crate::ast::{Expression, FunctionDefinition, Program, Statement};
use crate::lexer::{KeywordType, Token, TokenType};
use std::process::exit;

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut index: usize = 0;
    let function = parse_function(&tokens, &mut index);
    let program = Program { function };
    println!("{:?}", program);
    program
}

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> FunctionDefinition {
    // keyword type should have another enum that represent types?
    expect(TokenType::Keyword(KeywordType::Int), tokens, index);

    let name = expect(TokenType::Identifier, tokens, index).token_value;
    expect(TokenType::OParen, tokens, index);
    // implement arguments
    expect(TokenType::Identifier, tokens, index);
    expect(TokenType::CParen, tokens, index);
    expect(TokenType::OBrace, tokens, index);

    // statement
    let statement = parse_statement(tokens, index);
    expect(TokenType::CBrace, tokens, index);

    FunctionDefinition {
        name,
        body: statement,
    }
}

fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    expect(TokenType::Keyword(KeywordType::Return), tokens, index);
    let expression = parse_expression(tokens, index);
    expect(TokenType::SemiColon, tokens, index);
    Statement::Return(expression)
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let t = expect(TokenType::Constant, tokens, index);
    Expression::Constant(t.token_value.parse::<u32>().unwrap())
}

fn expect(token_type: TokenType, tokens: &Vec<Token>, index: &mut usize) -> Token {
    match tokens.get(*index) {
        Some(t) => {
            if token_type == t.token_type {
                *index += 1;
                t.clone()
            } else {
                eprintln!(
                    "Syntax error > expected {:?} found {:?}",
                    token_type, t.token_type
                );
                exit(1);
            }
        }
        None => {
            todo!()
        }
    }
}
