use parser::AstExpr;
use parser::AstOp;
use parser::AstExFn;

pub fn eval(expr: AstExpr) -> f64 {
    match expr {
        AstExpr::Literal(n) => n,
        AstExpr::Binary(op, l, r) => {
            match op {
                AstOp::Add => eval(*l) + eval(*r),
                AstOp::Sub => eval(*l) - eval(*r),
                AstOp::Mul => eval(*l) * eval(*r),
                AstOp::Div => eval(*l) / eval(*r),
                AstOp::Pow => f64::powf(eval(*l), eval(*r))
            }
        },
        AstExpr::External(ex, mut args) => {
            match ex {
                AstExFn::Sin => {
                    if args.len() != 1 {
                        panic!("Wrong number of arguments for external function sin, is {}, should be 1", args.len())
                    }
                    f64::sin(eval(*(args.remove(0))))
                },
                AstExFn::Cos => {
                    if args.len() != 1 {
                        panic!("Wrong number of arguments for external function cos, is {}, should be 1", args.len())
                    }
                    f64::cos(eval(*(args.remove(0))))
                },
                AstExFn::Tan => {
                    if args.len() != 1 {
                        panic!("Wrong number of arguments for external function tan, is {}, should be 1", args.len())
                    }
                    f64::tan(eval(*(args.remove(0))))
                },
                AstExFn::ASin => {
                    if args.len() != 1 {
                        panic!("Wrong number of arguments for external function asin, is {}, should be 1", args.len())
                    }
                    f64::asin(eval(*(args.remove(0))))
                },
                AstExFn::ACos => {
                    if args.len() != 1 {
                        panic!("Wrong number of arguments for external function acos, is {}, should be 1", args.len())
                    }
                    f64::acos(eval(*(args.remove(0))))
                },
                AstExFn::ATan2 => {
                    if args.len() != 2 {
                        panic!("Wrong number of arguments for external function atan2, is {}, should be 2", args.len())
                    }
                    f64::atan2(eval(*(args.remove(0))), eval(*(args.remove(0))))
                },
                AstExFn::ATan => {
                    if args.len() == 1 {
                        f64::atan(eval(*(args.remove(0))))
                    } else if args.len() == 2 {
                        f64::atan2(eval(*(args.remove(0))), eval(*(args.remove(0))))
                    } else {
                        panic!("Wrong number of arguments for external function atan, is {}, should be 1 or 2", args.len())
                    }
                    
                }
            }
        },
        AstExpr::Unary(op, val) => {
            match op {
                AstOp::Add => eval(*val),
                AstOp::Sub => - eval(*val),
                _ => panic!("Invalid unary operator {:?}", op)
            }
        }
    }
}