#[derive(Debug, PartialEq)]
pub struct StatementBlock {
    stmts: Vec<Box<Statement>>,
}

impl StatementBlock {
    pub fn new(stmts: Vec<Box<Statement>>) -> Self {
        StatementBlock { stmts }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign(String, opcode::Assign, Box<Expression>),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryOp(Box<Expression>, opcode::Binary, Box<Expression>),
    Identifier(String),
    Number(f64),
    FunctionCall(String, Vec<Box<Expression>>),
}

mod opcode {
    #[derive(Debug, PartialEq)]
    pub enum Binary {
        Add,
        Sub,
        Mul,
        Div,
        Eq,
    }

    #[derive(Debug, PartialEq)]
    pub enum Assign {
        Eq,
        Add,
        Sub,
        Mul,
        Div,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::opcode::*;
    use lalrpop_util::*;

    lalrpop_mod!(pub grammar);

    #[test]
    fn expression() {
        let src = "4.0 + (10.3 * x) / 2.5;";
        let parsed = grammar::StatementParser::new().parse(src).unwrap();

        let expected = Box::new(Statement::Expression(Box::new(Expression::BinaryOp(
            Box::new(Expression::Number(4.0)),
            Binary::Add,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::BinaryOp(
                    Box::new(Expression::Number(10.3)),
                    Binary::Mul,
                    Box::new(Expression::Identifier(String::from("x"))),
                )),
                Binary::Div,
                Box::new(Expression::Number(2.5)),
            )),
        ))));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn statement_expression() {
        let src = "4 + 5;";
        let parsed = grammar::StatementParser::new().parse(src).unwrap();

        let expected = Box::new(Statement::Expression(Box::new(Expression::BinaryOp(
            Box::new(Expression::Number(4.0)),
            Binary::Add,
            Box::new(Expression::Number(5.0)),
        ))));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn statement_assign() {
        let src = "x = 4 + 5;";
        let parsed = grammar::StatementParser::new().parse(src).unwrap();

        let expected = Box::new(Statement::Assign(
            String::from("x"),
            Assign::Eq,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number(4.0)),
                Binary::Add,
                Box::new(Expression::Number(5.0)),
            )),
        ));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn statement_block() {
        let src = "={ x = 5; x += 5; }";
        let parsed = grammar::StatementBlockParser::new().parse(src).unwrap();

        let expected = StatementBlock::new(vec![
            Box::new(Statement::Assign(
                String::from("x"),
                Assign::Eq,
                Box::new(Expression::Number(5.0)),
            )),
            Box::new(Statement::Assign(
                String::from("x"),
                Assign::Add,
                Box::new(Expression::Number(5.0)),
            )),
        ]);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn expression_function_call() {
        let src = "yep = 5 * sin(8.0) + cos(x);";
        let parsed = grammar::StatementParser::new().parse(src).unwrap();

        let expected = Box::new(Statement::Assign(
            String::from("yep"),
            Assign::Eq,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::BinaryOp(
                    Box::new(Expression::Number(5.0)),
                    Binary::Mul,
                    Box::new(Expression::FunctionCall(
                        String::from("sin"),
                        vec![Box::new(Expression::Number(8.0))],
                    )),
                )),
                Binary::Add,
                Box::new(Expression::FunctionCall(
                    String::from("cos"),
                    vec![Box::new(Expression::Identifier(String::from("x")))],
                )),
            )),
        ));

        assert_eq!(parsed, expected);
    }
}
