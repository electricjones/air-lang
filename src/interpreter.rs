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

impl Compile for Interpreter {
    type Output = anyhow::Result<i32>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret = 0i32;
        let evaluator = Eval::new();
        for node in ast {
            ret += evaluator.eval(&node);
        }
        Ok(ret)
    }
}

pub struct Interpreter {}

pub struct Eval {}

impl Eval {
    pub fn new() -> Self {
        Eval {}
    }

    pub fn eval(&self, node: &Node) -> i32 {
        match node {
            Node::Int(n) => *n,
            Node::UnaryExpr { operator, child } => {
                let child = self.eval(child);
                match operator {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                }
            }
            Node::BinaryExpr { operator, left, right } => {
                let left_ret = self.eval(left);
                let right_ret = self.eval(right);

                match operator {
                    Operator::Plus => left_ret + right_ret,
                    Operator::Minus => left_ret - right_ret,
                }
            }
        }
    }
}
