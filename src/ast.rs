//program = Program(function_definition)
//function_definition = Function(identifier name, statement body)
//statement = Return(exp)
//exp = Constant(int)

#[derive(Debug)]
pub struct Program {
    pub function: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(u32),
}
