//program = Program(function_definition)
//function_definition = Function(identifier name, instruction* instructions)
//instruction = Mov(operand src, operand dst) | Ret
//operand = Imm(int) | Register
use crate::ast::{Expression, FunctionDefinition, Program, Statement};

#[derive(Debug)]
pub struct ASMProgram {
    pub function: ASMFunctionDefinition,
}

#[derive(Debug)]
pub struct ASMFunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(ASMStatement),
    Ret,
}

#[derive(Debug, Clone)]
pub struct ASMStatement {
    pub src: Operand,
    pub dst: Operand,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(u32),
    Register(Register),
}

#[derive(Debug, Clone)]
pub enum Register {
    EAX,
}

pub fn to_asm(ast: Program) -> ASMProgram {
    let fn_def = to_asm_fn(ast.function);
    ASMProgram { function: fn_def }
}

fn to_asm_fn(ast_fn: FunctionDefinition) -> ASMFunctionDefinition {
    let instructions = to_asm_instructions(ast_fn.body);
    ASMFunctionDefinition {
        name: ast_fn.name,
        instructions,
    }
}

fn to_asm_instructions(ast_stm: Statement) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    match ast_stm {
        Statement::Return(expr) => {
            instructions.push(to_asm_instruction(expr));
            instructions.push(Instruction::Ret);
        }
    }
    instructions
}
fn to_asm_instruction(ast_exp: Expression) -> Instruction {
    let statement = match ast_exp {
        Expression::Constant(val) => to_asm_statement(val),
    };
    Instruction::Mov(statement)
}

fn to_asm_statement(val: u32) -> ASMStatement {
    //naive implementation but for now, it should work
    ASMStatement {
        dst: Operand::Register(Register::EAX),
        src: Operand::Imm(val),
    }
}
