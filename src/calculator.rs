#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char)
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        // ascii so sub 48 to get 0 through 9 val
                        *n = *n * 10 + (c as u32 - 48);
                    },
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                },
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '-' => tokens.push(Token::Op(Operator::Sub)),

                // whitespace
                ' ' => {},
                '\n' => {},
                _ => return Err(Error::BadToken(c))

            }
        }
        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }
        Ok(tokens)
    }
}
