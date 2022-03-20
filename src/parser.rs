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
pub struct AirParser;

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
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}


impl Node {
    fn from_expression(pair: Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::Expr => Node::from_expression(pair.into_inner().next().unwrap()),
            Rule::UnaryExpr => {
                let mut pair = pair.into_inner();
                let op = pair.next().unwrap();
                let child = pair.next().unwrap();
                let child = Node::from_term(child);
                Node::from_unary_expression(op, child)
            }
            Rule::BinaryExpr => {
                let mut pair = pair.into_inner();
                let lhspair = pair.next().unwrap();
                let mut lhs = Node::from_term(lhspair);
                let mut op = pair.next().unwrap();
                let rhspair = pair.next().unwrap();
                let mut rhs = Node::from_term(rhspair);
                let mut retval = Node::from_binary_expression(op, lhs, rhs);
                loop {
                    let pair_buf = pair.next();
                    if pair_buf != None {
                        op = pair_buf.unwrap();
                        lhs = retval;
                        rhs = Node::from_term(pair.next().unwrap());
                        retval = Node::from_binary_expression(op, lhs, rhs);
                    } else {
                        return retval;
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
            op: match pair.as_str() {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                _ => unreachable!(),
            },
            child: Box::new(child),
        }
    }

    // fn Node::from_binary_expression(pair: Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    fn from_binary_expression(pair: Pair<Rule>, lhs: Node, rhs: Node) -> Node {
        Node::BinaryExpr {
            op: match pair.as_str() {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                _ => unreachable!(),
            },
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
