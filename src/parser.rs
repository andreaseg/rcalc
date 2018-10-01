use scanner::Token;
use std::str::FromStr;

/*
Parsing rules
expr := add
add := mul (( + | - )) add)*
mul := pow (( * | / ) pow)*
pow := unary (^ unary)*
unary := (-) unary | primary
primary := (primary) | number | external
*/

#[derive(PartialEq, Debug)]
pub enum AstExpr {
    Literal(f64),
    Binary(AstOp, Box<AstExpr>, Box<AstExpr>),
    External(AstExFn, Vec<Box<AstExpr>>),
    Unary(AstOp, Box<AstExpr>)
}

#[derive(PartialEq, Eq, Debug)]
pub enum AstOp {
    Add,    // a + b
    Sub,    // a - b
    Mul,    // a * b
    Div,    // a / b
    Pow     // a ^ b
}

impl FromStr for AstOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "+"  => Ok(AstOp::Add),
            "-"  => Ok(AstOp::Sub),
            "*"  => Ok(AstOp::Mul),
            "/"  => Ok(AstOp::Div),
            "^"  => Ok(AstOp::Pow),
            _    => Err(())
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum AstExFn {
    Sin,
    Cos,
    ASin,
    ACos,
    Tan,
    ATan,
    ATan2
}

impl FromStr for AstExFn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sin"   => Ok(AstExFn::Sin),
            "cos"   => Ok(AstExFn::Cos),
            "asin"  => Ok(AstExFn::ASin),
            "acos"  => Ok(AstExFn::ACos),
            "tan"   => Ok(AstExFn::Tan),
            "atan"  => Ok(AstExFn::ATan),
            "atan2" => Ok(AstExFn::ATan2),
            _       => Err(())
        }
    }
}


pub fn parse(mut tokens: Vec<Token>) -> AstExpr {
    tokens.reverse();
    let ast = parse_expr(&mut tokens);

    if !tokens.is_empty() {
        tokens.reverse();
        panic!("Unconsumed tokens {:?}", tokens);
    }
    ast
}

fn parse_expr(tokens: &mut Vec<Token>) -> AstExpr {
    parse_add(tokens)
}

fn parse_add(tokens: &mut Vec<Token>) -> AstExpr {
    let mut node = parse_mul(tokens);
    while let Some(token) = tokens.pop() {
        node = match token {
            Token::Operator(op) => {
                match op.as_ref() {
                    "+"  => AstExpr::Binary(AstOp::Add, Box::new(node), Box::new(parse_mul(tokens))),
                    "-" => AstExpr::Binary(AstOp::Sub, Box::new(node), Box::new(parse_mul(tokens))),
                    other => {
                        tokens.push(Token::Operator(other.to_string()));
                        break;
                    }
                }
            },
            other => {
                tokens.push(other);
                break;
            }
        }
    };
    node
}

fn parse_mul(tokens: &mut Vec<Token>) -> AstExpr {
    let mut node = parse_pow(tokens);
    while let Some(token) = tokens.pop() {
        node = match token {
            Token::Operator(op) => {
                match op.as_ref() {
                    "*" => AstExpr::Binary(AstOp::Mul, Box::new(node), Box::new(parse_pow(tokens))),
                    "/" => AstExpr::Binary(AstOp::Div, Box::new(node), Box::new(parse_pow(tokens))),
                    other => {
                        tokens.push(Token::Operator(other.to_string()));
                        break;
                    }
                }
            },
            other => {
                tokens.push(other);
                break;
            }
        }
    }
    node
}

fn parse_pow(tokens: &mut Vec<Token>) -> AstExpr {
    let mut node = parse_unary(tokens);
    while let Some(token) = tokens.pop() {
        node = match token {
            Token::Operator(op) => {
                match op.as_ref() {
                    "^" => AstExpr::Binary(AstOp::Pow, Box::new(node), Box::new(parse_unary(tokens))),
                    other => {
                        tokens.push(Token::Operator(other.to_string()));
                        break;
                    }
                }
            },
            other => {
                tokens.push(other);
                break;
            }
        }
    }
    node
}

fn parse_unary(tokens: &mut Vec<Token>) -> AstExpr {
    match tokens.pop().expect("Expected unary token") {
        Token::Operator(op) => {
            match op.as_ref() {
                "-" => AstExpr::Unary(AstOp::Sub, Box::new(parse_primary(tokens))),
                "+" => parse_primary(tokens),
                other => panic!("Unexpected token {:?}, should be - or +", other)
            }
        }
        other => {
            tokens.push(other);
            parse_primary(tokens)
        }
    }
}

fn parse_primary(tokens: &mut Vec<Token>) -> AstExpr {
    match tokens.pop().expect("Expected expression") {
        Token::Number(n) => AstExpr::Literal(n),
        Token::LeftPar => {
            let ast_node = parse_expr(tokens);
            match tokens.pop().expect("Expected token") {
                Token::RightPar => ast_node,
                _ => panic!("Expected )")
            }
        },
        Token::External(name) => {
            let mut args = Vec::new();
            loop {
                let ast_node = parse_expr(tokens);
                match tokens.pop().expect("Expected token") {
                    Token::Comma => args.push(Box::new(ast_node)),
                    Token::RightPar => {
                        args.push(Box::new(ast_node));
                        break;
                    },
                    _ => panic!("Expected , or )")
                }
            }
            AstExpr::External(
                AstExFn::from_str(&name).expect("Unknown external function"),
                args
            )
        },
        other => panic!("Unexpected token {:?}", other)
    }
}




#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn parse_external() {
        assert_eq!(AstExFn::Sin, AstExFn::from_str("Sin").unwrap());
        assert_eq!(AstExFn::ASin, AstExFn::from_str("ASin").unwrap());
        assert_eq!(AstExFn::Cos, AstExFn::from_str("Cos").unwrap());
        assert_eq!(AstExFn::ACos, AstExFn::from_str("ACos").unwrap());
        assert_eq!(AstExFn::Tan, AstExFn::from_str("Tan").unwrap());
        assert_eq!(AstExFn::ATan, AstExFn::from_str("ATan").unwrap());
        assert_eq!(AstExFn::ATan2, AstExFn::from_str("ATan2").unwrap());
    }

    #[test]
    fn single_literal_expr() {
        assert_eq!(parse(vec![Token::Number(1.0)]), AstExpr::Literal(1.0))
    }

    #[test]
    fn binary_expr() {
        assert_eq!(parse(::scanner::tokenize("1 + 2".to_string())), AstExpr::Binary(AstOp::Add, Box::new(AstExpr::Literal(1.0)), Box::new(AstExpr::Literal(2.0))));
        assert_eq!(parse(::scanner::tokenize("1 * 2".to_string())), AstExpr::Binary(AstOp::Mul, Box::new(AstExpr::Literal(1.0)), Box::new(AstExpr::Literal(2.0))));
        assert_eq!(parse(::scanner::tokenize("1 / 2".to_string())), AstExpr::Binary(AstOp::Div, Box::new(AstExpr::Literal(1.0)), Box::new(AstExpr::Literal(2.0))));
        assert_eq!(parse(::scanner::tokenize("1 - 2".to_string())), AstExpr::Binary(AstOp::Sub, Box::new(AstExpr::Literal(1.0)), Box::new(AstExpr::Literal(2.0))));
        assert_eq!(parse(::scanner::tokenize("1 ^ 2".to_string())), AstExpr::Binary(AstOp::Pow, Box::new(AstExpr::Literal(1.0)), Box::new(AstExpr::Literal(2.0))));
    }

    #[test]
    fn test_par() {
        assert_eq!(parse(::scanner::tokenize("(1)".to_string())), AstExpr::Literal(1.0));
    }

    #[test]
    fn test_par2() {
        assert_eq!(parse(::scanner::tokenize("(1) + (2)".to_string())), 
        AstExpr::Binary(AstOp::Add, Box::new(AstExpr::Literal(1.0)), Box::new(AstExpr::Literal(2.0))));
    }

    #[test]
    fn test_order() {
        assert_eq!(
            parse(::scanner::tokenize("1 + 2 * 3".to_string())),
            AstExpr::Binary(
                AstOp::Add,
                Box::new(AstExpr::Literal(1.0)),
                Box::new(
                    AstExpr::Binary(
                        AstOp::Mul,
                        Box::new(AstExpr::Literal(2.0)),
                        Box::new(AstExpr::Literal(3.0))
                    )
                )
            )
        );

        assert_eq!(
            parse(::scanner::tokenize("1 * 2 + 3".to_string())),
            AstExpr::Binary(
                AstOp::Add,
                Box::new(
                    AstExpr::Binary(
                        AstOp::Mul,
                        Box::new(AstExpr::Literal(1.0)),
                        Box::new(AstExpr::Literal(2.0))
                    )
                ),
                Box::new(AstExpr::Literal(3.0))
            )
        );
    }

}