use crate::statement::Statement;

pub fn interpret_program(statements: Vec<Statement>) {
    for mut statement in statements {
        statement.evaluate();
    }
}
