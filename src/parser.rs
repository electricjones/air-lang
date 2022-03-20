use pest::iterators::Pair;
use pest::Parser;

pub type ParserResult = Result<Vec<Node>, pest::error::Error<Rule>>;

pub fn parse(source: &str) -> ParserResult {
    let mut ast = vec![];
    let pairs = AirParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(Node::from_expression(pair));
        }
    }

    Ok(ast)
}

#[derive(pest_derive::Parser)]
#[grammar = "air-grammar.pest"]
struct AirParser;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
}

// @todo: rename to fully qualified words
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
            unknown => panic!("Unknown expr: {:?}", unknown),
        }
    }

    fn from_term(pair: Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::Int => {
                let istr = pair.as_str();
                let (sign, istr) = match &istr[..1] {
                    "-" => (-1, &istr[1..]),
                    _ => (1, istr),
                };
                let int: i32 = istr.parse().unwrap();
                Node::Int(sign * int)
            }
            Rule::Expr => Node::from_expression(pair),
            unknown => panic!("Unknown term: {:?}", unknown),
        }
    }

    fn from_unary_expression(pair: Pair<Rule>, child: Node) -> Node {
        Node::UnaryExpr {
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
                _ => unreachable!(),
            },
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}
