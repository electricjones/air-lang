use std::fmt;

use pest::iterators::Pair;
use pest::Parser;
use regex::Regex;

pub type Ast = Vec<Node>;
pub type ParserResult = Result<Ast, pest::error::Error<Rule>>;

pub trait AirLangParser {
    fn parse_source(&self, source: &str) -> ParserResult;
    fn pre_process(&self, source: &str) -> String {
        String::from(source)
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "air-grammar.pest"]
pub struct StandardParser;

impl StandardParser {
    pub fn new() -> Self {
        StandardParser
    }
}

impl AirLangParser for StandardParser {
    fn parse_source(&self, source: &str) -> ParserResult {
        let mut ast = vec![];
        let pairs = StandardParser::parse(Rule::Program, source)?;
        for pair in pairs {
            if let Rule::Expr = pair.as_rule() {
                ast.push(Node::from_expression(pair));
            }
        }

        Ok(ast)
    }

    fn pre_process(&self, source: &str) -> String {
        // Group exponents
        let exponent_regex = Regex::new(r"([-+]?[0-9])\s?\^\s?([0-9])").unwrap();
        let processed = exponent_regex.replace_all(source, "($1^$2)");

        // Build our final string and add a +0 to everything so its always a BinaryExpr
        let mut response = String::from(processed);
        response.push_str(" + 0");
        response

    }
}

#[derive(Debug)]
pub enum
Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
}

impl fmt::Display for Operator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(formatter, "+"),
            Operator::Minus => write!(formatter, "-"),
            Operator::Multiply => write!(formatter, "*"),
            Operator::Divide => write!(formatter, "/"),
            Operator::Exponent => write!(formatter, "^"),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Int(i32),
    UnaryExpr {
        operator: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        operator: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    fn from_expression(pair: Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::Expr => Node::from_expression(pair.into_inner().next().unwrap()),
            Rule::UnaryExpr => {
                let mut pair = pair.into_inner();
                let operator = pair.next().unwrap();
                let child = pair.next().unwrap();
                let child = Node::from_term(child);
                Node::from_unary_expression(operator, child)
            }
            Rule::BinaryExpr => {
                let mut pair = pair.into_inner();

                let left_pair = pair.next().unwrap();
                let mut left = Node::from_term(left_pair);

                let mut operator = pair.next().unwrap();

                let right_pair = pair.next().unwrap();
                let mut right = Node::from_term(right_pair);

                let mut return_value = Node::from_binary_expression(operator, left, right);

                loop {
                    let pair_buf = pair.next();
                    if pair_buf != None {
                        operator = pair_buf.unwrap();
                        left = return_value;
                        right = Node::from_term(pair.next().unwrap());
                        return_value = Node::from_binary_expression(operator, left, right);
                    } else {
                        return return_value;
                    }
                }
            }
            unknown => panic!("Unknown expression: {:?}", unknown),
        }
    }

    fn from_term(pair: Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::Int => {
                let pair_string = pair.as_str();
                let (sign, pair_string) = match &pair_string[..1] {
                    "-" => (-1, &pair_string[1..]),
                    _ => (1, pair_string),
                };
                let integer: i32 = pair_string.parse().unwrap();
                Node::Int(sign * integer)
            }
            Rule::Expr => Node::from_expression(pair),
            unknown => panic!("Unknown term: {:?}", unknown),
        }
    }

    fn from_unary_expression(pair: Pair<Rule>, child: Node) -> Node {
        Node::UnaryExpr {
            // @todo: This pattern is repeated a lot
            operator: match pair.as_str() {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                _ => unreachable!(),
            },
            child: Box::new(child),
        }
    }

    fn from_binary_expression(pair: Pair<Rule>, left: Node, right: Node) -> Node {
        Node::BinaryExpr {
            operator: match pair.as_str() {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                "^" => Operator::Exponent,
                _ => unreachable!()
            },
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Int(value) => write!(formatter, "{}", value),

            Node::UnaryExpr {
                operator,
                child
            } => write!(formatter, "{}{}", operator, child),

            Node::BinaryExpr {
                operator,
                left,
                right
            } => write!(formatter, "{} {} {}", left, operator, right),
        }
    }
}
