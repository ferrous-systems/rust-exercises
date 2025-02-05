#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Square(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotANumber(String),
    MissingOperand,
    TooManyOperands,
    EmptyInput,
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Box<Expr>> = Vec::new();
    for word in input.split_ascii_whitespace() {
        let expr = match word {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or(ParseError::MissingOperand)?;
                let a = stack.pop().ok_or(ParseError::MissingOperand)?;
                match word {
                    "+" => Expr::Add(a, b),
                    "-" => Expr::Sub(a, b),
                    "*" => Expr::Mul(a, b),
                    "/" => Expr::Div(a, b),
                    _ => unreachable!(),
                }
            }
            "sqr" => {
                let a = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Square(a)
            }
            _ => {
                let number = word
                    .parse::<i64>()
                    .map_err(|_| ParseError::NotANumber(word.to_string()))?;
                Expr::Number(number)
            }
        };
        stack.push(Box::new(expr));
    }

    match stack.pop() {
        Some(expr) if stack.is_empty() => Ok(*expr),
        Some(_) => Err(ParseError::TooManyOperands),
        None => Err(ParseError::EmptyInput),
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    DivisionByZero,
}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    let value = match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a)? + eval(b)?,
        Expr::Sub(a, b) => eval(a)? - eval(b)?,
        Expr::Mul(a, b) => eval(a)? * eval(b)?,
        Expr::Div(_, divisor) if eval(divisor)? == 0 => return Err(EvalError::DivisionByZero),
        Expr::Div(a, b) => eval(a)? / eval(b)?,
        Expr::Square(a) => eval(a)?.pow(2),
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let input = "42";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn not_a_number() {
        let input = "X";
        let error = parse(input).unwrap_err();
        assert_eq!(error, ParseError::NotANumber("X".to_string()));
    }

    #[test]
    fn add() {
        let input = "40 2 +";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn subtract() {
        let input = "42 2 -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 40);
    }

    #[test]
    fn multiply() {
        let input = "21 2 *";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn divide() {
        let input = "84 2 /";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn divide_by_zero() {
        let input = "42 0 /";
        let expr = parse(input).unwrap();
        let error = eval(&expr).unwrap_err();
        assert_eq!(error, EvalError::DivisionByZero);
    }

    #[test]
    fn square() {
        let input = "5 sqr";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 25);
    }

    #[test]
    fn missing_operand() {
        let input = "42 +";
        let error = parse(input).unwrap_err();
        assert_eq!(error, ParseError::MissingOperand);
    }

    #[test]
    fn too_many_operands() {
        let input = "42 42 42 +";
        let error = parse(input).unwrap_err();
        assert_eq!(error, ParseError::TooManyOperands);
    }

    #[test]
    fn empty_input() {
        let input = "      ";
        let error = parse(input).unwrap_err();
        assert_eq!(error, ParseError::EmptyInput);
    }

    #[test]
    fn smoke_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 0);
    }
}
