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
        let pre_processed = self.parser.pre_process(&source);
        let ast = self.parser.parse_source(pre_processed.as_str()).unwrap();

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
                    Operator::Exponent => child,
                    Operator::Divide => child,
                }
            }
            Node::BinaryExpr { operator, left, right } => {
                let left_value = self.evaluate(left);
                let right_value = self.evaluate(right);

                match operator {
                    Operator::Plus => left_value + right_value,
                    Operator::Minus => left_value - right_value,
                    Operator::Multiply => left_value * right_value,
                    Operator::Divide => {
                        let float: f32 = left_value as f32 / right_value as f32;
                        float.round() as i32
                    },
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
