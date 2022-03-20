use anyhow::Error;

use crate::parser::{AirParser, Node};
use crate::parser::Operator;

pub enum ExecutionResult {
    Valid(i32),
    Invalid(Error),
}

pub struct Interpreter {
    parser: AirParser,
    evaluator: Evaluator,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            parser: AirParser::new(), // @todo:  As a trait
            evaluator: Evaluator::new(), // @todo: As a trait
        }
    }

    pub fn execute(&self, source: &str) -> ExecutionResult {
        let ast = self.parser.parse_source(source).unwrap();

        let mut value = 0i32;
        for node in ast {
            value += self.evaluator.evaluate(&node);
        }

        // @todo: Handle invalid case
        ExecutionResult::Valid(value)
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
