use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiate,
}

#[derive(Debug)]
enum Token {
    // TODO this should be an i32, but tokenizer needs to be more mature for that
    Num(u32),
    Op(Operation),
}

fn precedence(op: &Operation) -> u8 {
    match op {
        Operation::Add => 1,
        Operation::Subtract => 1,
        Operation::Multiply => 2,
        Operation::Divide => 2,
        Operation::Exponentiate => 2,
    }
}

fn char_to_operator(c: char) -> Option<Operation> {
    match c {
        '+' => Some(Operation::Add),
        '-' => Some(Operation::Subtract),
        '*' => Some(Operation::Multiply),
        '/' => Some(Operation::Divide),
        '^' => Some(Operation::Exponentiate),
        _ => None,
    }
}

fn tokenize(code: &str) -> Option<Vec<Token>> {
    let chars: Vec<char> = code
        .chars()
        .into_iter()
        .filter(|c| !c.is_whitespace())
        .collect();

    if chars.len() == 0 {
        return None;
    }

    chars
        .into_iter()
        .map(|c| match c.is_digit(10) {
            true => c.to_digit(10).map(|num| Token::Num(num)),
            _ => char_to_operator(c).map(|op| Token::Op(op)),
        })
        .collect()
}

#[derive(Clone)]
enum Expression<'a> {
    BinOp(&'a Expression<'a>, Operation, &'a Expression<'a>),
    Num(u32),
}

fn pratt_parse(
    current_precedence: u8,
    tokens: &mut VecDeque<Token>,
) -> Option<Expression<'static>> {
    match tokens.pop_front()? {
        Token::Op(_) => None,
        Token::Num(num) => {
            let mut left = Expression::Num(num);
            while tokens.len() != 0 {
                match tokens.get(0).unwrap() {
                    Token::Op(op) => {
                        let prec = precedence(op);
                        if prec <= current_precedence {
                            return Some(left.clone());
                        }
                        tokens.pop_front();
                        let right = pratt_parse(prec, tokens)?;
                        left = Expression::BinOp(&left.clone(), *op, &right.clone());
                    }
                    Token::Num(_) => {
                        return Some(left.clone());
                    }
                }
            }
            Some(left.clone())
        }
    }
}

// const prattParse = (precedence: number): Expression => {
//   const maybeToken = tokens.shift();
//   if (maybeToken === undefined || typeof maybeToken !== "number") {
//     throw new Error("I should have found a number here...");
//   }

//   let left: Expression = maybeToken;

//   while (tokens[0] !== undefined && isOperation(tokens[0])) {
//     // we know the next token is an operator so we
//     // grab the precedence for it
//     const prec = OPERATORS[tokens[0]];

//     if (prec <= precedence) {
//       return left;
//     }

//     const op = tokens.shift();
//     if (typeof op === "number" || op === undefined) {
//       throw new Error("I expected an operation here");
//     }

//     const right = prattParse(prec);
//     left = { left, op, right };
//   }
//   return left;
// };

// return prattParse(0);
// };

fn parse(tokens: Vec<Token>) -> Option<Expression<'static>> {
    if tokens.len() == 0 {
        return None;
    }

    let mut token_deque = VecDeque::from(tokens);

    return pratt_parse(0, &mut token_deque);
}

fn main() {
    println!("{:#?}", tokenize("1 + 2 + 3"));
}
