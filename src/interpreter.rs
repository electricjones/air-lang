use crate::parser::Node;
use crate::parser::Operator;

pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        // println!("Compiling the source: {}", source);
        let ast: Vec<Node> = crate::parser::parse(source).unwrap();
        // println!("{:?}", ast);
        Self::from_ast(ast)
    }
}

pub struct Interpreter {}

impl Compile for Interpreter {
    type Output = anyhow::Result<i32>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut value = 0i32;
        let evaluator = Evaluator::new();
        for node in ast {
            value += evaluator.evaluate(&node);
        }
        Ok(value)
    }
}

struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn evaluate(&self, node: &Node) -> i32 {
        match node {
            Node::Int(n) => *n,
            Node::UnaryExpr { operator, child } => {
                let child = self.evaluate(child);
                match operator {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                }
            }
            Node::BinaryExpr { operator, left, right } => {
                let left_value = self.evaluate(left);
                let right_value = self.evaluate(right);

                match operator {
                    Operator::Plus => left_value + right_value,
                    Operator::Minus => left_value - right_value,
                }
            }
        }
    }
}
