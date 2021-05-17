use crate::expr::Expr;

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    Expr(Expr),
}

impl Statement {
    pub fn evaluate(&mut self) {
        match self {
            Statement::Print(expr) => {
                println!("{}", expr.evaluate().to_string().unwrap());
            }

            Statement::Expr(expr) => {
                expr.evaluate();
            }
        }
    }
}
