#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens,
    DivisionByZero,
    InvalidExpression,
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
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                }
                '+' => tokens.push(Token::Op(Operator::Add)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                ' ' => {}
                '\n' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }
        if !parens.is_empty() {
            return Err(Error::MismatchedParens);
        }
        Ok(tokens)
    }

    pub fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
        let mut output_queue: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        for token in tokens {
            match token {
                Token::Number(_) => output_queue.push(token),
                Token::Op(op) => {
                    while let Some(Token::Op(top_op)) = operator_stack.last() {
                        if Self::precedence(&op) <= Self::precedence(top_op) {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(Token::Op(op));
                }
                Token::Bracket('(') => operator_stack.push(token),
                Token::Bracket(')') => {
                    while let Some(top) = operator_stack.pop() {
                        if top == Token::Bracket('(') {
                            break;
                        }
                        output_queue.push(top);
                    }
                }
                _ => {} // if parsing is right then this should not happen
            }
        }

        while let Some(op) = operator_stack.pop() {
            output_queue.push(op);
        }

        output_queue
    }

    fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::Add | Operator::Sub => 1,
            Operator::Mul | Operator::Div => 2,
        }
    }

    pub fn evaluate_rpn(tokens: Vec<Token>) -> Result<u32, Error> {
        let mut stack: Vec<u32> = Vec::new();

        for token in tokens {
            match token {
                Token::Number(n) => stack.push(n),
                Token::Op(op) => {
                    if stack.len() < 2 {
                        return Err(Error::InvalidExpression);
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = match op {
                        Operator::Add => a + b,
                        Operator::Sub => a - b,
                        Operator::Mul => a * b,
                        Operator::Div => {
                            if b == 0 {
                                return Err(Error::DivisionByZero);
                            }
                            a / b
                        }
                    };
                    stack.push(result);
                }
                _ => return Err(Error::InvalidExpression),
            }
        }

        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        } else {
            Err(Error::InvalidExpression)
        }
    }

    pub fn calculate<T: AsRef<str>>(expr: T) -> Result<u32, Error> {
        let tokens = Self::parse(expr)?;
        let rpn_tokens = Self::to_rpn(tokens);
        Self::evaluate_rpn(rpn_tokens)
    }
}
