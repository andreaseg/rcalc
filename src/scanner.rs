use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub enum Token {
    LeftPar,
    RightPar,
    Number(f64),
    Operator(String),
    Comma,
    External(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::LeftPar => write!(f, "("),
            Token::RightPar => write!(f, ")"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Operator(op) => write!(f, "{}", op),
            Token::Comma => write!(f, ","),
            Token::External(ext) => write!(f, "{}", ext),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let re = Regex::new(concat!(
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<lpar>\()|",
        r"(?P<rpar>\))|",
        r"(?P<external>[[:alpha:]][[:alnum:]]*\()|",
        r"(?P<comma>,)|",
        r"(?P<operator>[\+|\-|\*|/|\^])"
    )).unwrap();

    for cap in re.captures_iter(&input) {
        let token = if cap.name("number").is_some() {
            match cap.name("number").unwrap().as_str().parse() {
                Ok(n) => Token::Number(n),
                Err(e) => panic!("Lexer failed trying to parse number du to error {}", e),
            }
        } else if cap.name("external").is_some() {
            let mut s = cap.name("external").unwrap().as_str().to_string();
            let l = s.len();
            s.truncate(l - 1);
            Token::External(s)
        } else if cap.name("lpar").is_some() {
            Token::LeftPar
        } else if cap.name("rpar").is_some() {
            Token::RightPar
        } else if cap.name("operator").is_some() {
            Token::Operator(cap.name("operator").unwrap().as_str().to_string())
        } else if cap.name("comma").is_some() {
            Token::Comma
        } else {
            panic!("Unable to parse expression");
        };

        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! test_tokenizer {
        ($name:ident, $match_values:expr, $match_pattern:pat, $err_msg:expr) => {
            #[test]
            fn $name() {
                let tokens = tokenize($match_values.to_string());

                if tokens.len() == 0 {
                    panic!("No tokens found: {}", $err_msg)
                }

                for t in tokens {
                    match t {
                        $match_pattern => continue,
                        _ => panic!("Invalid token '{:?}' : {}", t, $err_msg),
                    }
                }
            }
        };
    }

    test_tokenizer!(
        number,
        "2",
        Token::Number(_),
        "Tokenizer does not recognize numbers"
    );
    test_tokenizer!(
        lpar,
        "(",
        Token::LeftPar,
        "Tokenizer does not recognize '('"
    );
    test_tokenizer!(
        rpar,
        ")",
        Token::RightPar,
        "Tokenizer does not recognize ')'"
    );
    test_tokenizer!(
        operator,
        "+-*/^",
        Token::Operator(_),
        "Tokenizer does not recognize operators"
    );
    test_tokenizer!(
        comma,
        ",",
        Token::Comma,
        "Tokenizer does not recognize comma."
    );
    test_tokenizer!(
        external,
        "sin(",
        Token::External(_),
        "Tokenizer does not recognize externals."
    );

}
