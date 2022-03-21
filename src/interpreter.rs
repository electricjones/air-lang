use crate::parser::{AirLangParser, Node, Operator, StandardParser};

pub type ExecutionResult = anyhow::Result<i32>;

pub struct Interpreter<> {
    parser: Box<dyn AirLangParser>,
    evaluator: Box<dyn Evaluator>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            parser: Box::new(StandardParser::new()),
            evaluator: Box::new(StandardEvaluator::new()),
        }
    }

    pub fn execute(&self, source: &str) -> ExecutionResult {
        let ast = self.parser.parse_source(source).unwrap();

        let mut value = 0i32;
        for node in ast {
            value += self.evaluator.evaluate(&node);
        }

        Ok(value)
    }
}

pub trait Evaluator {
    fn evaluate(&self, node: &Node) -> i32 {
        match node {
            Node::Int(n) => *n,
            Node::UnaryExpr { operator, child } => {
                let child = self.evaluate(child);
                match operator {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                    Operator::Multiply => child,
                    Operator::Exponent => child
                }
            }
            Node::BinaryExpr { operator, left, right } => {
                let left_value = self.evaluate(left);
                let right_value = self.evaluate(right);

                match operator {
                    Operator::Plus => left_value + right_value,
                    Operator::Minus => left_value - right_value,
                    Operator::Multiply => left_value * right_value,
                    Operator::Exponent => left_value.pow(right_value as u32),
                }
            }
        }
    }
}

struct StandardEvaluator {}

impl StandardEvaluator {
    pub fn new() -> Self {
        StandardEvaluator {}
    }
}

impl Evaluator for StandardEvaluator {}
