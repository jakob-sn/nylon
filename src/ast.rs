use visitor::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct TopNode {
    pub tree: Vec<TopLevel>,
}

impl Visitable for TopNode {
    fn visit(&self, visitor: &mut Visitor) {
        for i in &self.tree {
            i.visit(visitor);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    TextBlock(Box<TextBlock>),
    StatementBlock(Box<StatementBlock>),
}

impl Visitable for TopLevel {
    fn visit(&self, visitor: &mut Visitor) {
        match self {
            TopLevel::TextBlock(b) => {
                b.visit(visitor);
            }
            TopLevel::StatementBlock(b) => b.visit(visitor),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBlock {
    text: Vec<Text>,
}

impl Visitable for TextBlock {
    fn visit(&self, visitor: &mut Visitor) {
        for i in &self.text {
            i.visit(visitor);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Text {
    Text(String),
    InlineStatement(InlineStatement),
}

impl Visitable for Text {
    fn visit(&self, visitor: &mut Visitor) {
        match self {
            Text::Text(t) => {
                todo!()
            }
            Text::InlineStatement(stmt) => stmt.visit(visitor),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum InlineStatement {
    VariableCall(String),
    FunctionCall(String, Vec<Box<Expression>>),
}

impl Visitable for InlineStatement {
    fn visit(&self, visitor: &mut Visitor) {
        match self {
            InlineStatement::VariableCall(v) => {
                todo!()
            }
            InlineStatement::FunctionCall(id, args) => {
                todo!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatementBlock {
    stmts: Vec<Box<Statement>>,
}

impl Visitable for StatementBlock {
    fn visit(&self, visitor: &mut Visitor) {
        for i in &self.stmts {
            i.visit(visitor);
        }
    }
}

impl StatementBlock {
    pub fn new(stmts: Vec<Box<Statement>>) -> Self {
        StatementBlock { stmts }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assign(String, opcode::Assign, Box<Expression>),
    Expression(Box<Expression>),
}

impl Visitable for Statement {
    fn visit(&self, visitor: &mut Visitor) {
        match self {
            Statement::Assign(name, op, expr) => {
                todo!()
            }
            Statement::Expression(expr) => {
                todo!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    BinaryOp(Box<Expression>, opcode::Binary, Box<Expression>),
    Identifier(String),
    Number(f64),
    FunctionCall(String, Vec<Box<Expression>>),
}

impl Visitable for Expression {
    fn visit(&self, visitor: &mut Visitor) {
        match self {
            Expression::BinaryOp(left, op, right) => {
                todo!()
            }
            Expression::Identifier(id) => {
                todo!()
            }
            Expression::Number(n) => {
                todo!()
            }
            Expression::FunctionCall(id, args) => {
                todo!()
            }
        }
    }
}

mod opcode {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Binary {
        Add,
        Sub,
        Mul,
        Div,
        Eq,
    }

    #[derive(Debug, Clone, PartialEq)]
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
