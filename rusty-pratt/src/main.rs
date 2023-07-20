use std::collections::VecDeque;

#[derive(Debug)]
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

fn precedence(op: Operation) -> u8 {
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

enum Expression<'a> {
    BinOp(&'a Expression<'a>, Operation, &'a Expression<'a>),
    Num(u32)
}

fn parse(tokens: Vec<Token>) -> Option<Expression<'static>> {
    if tokens.len() == 0 {
        return None;
    }

    let token_deque = VecDeque::from(tokens);

    fn prattParse(precedence: u8) -> Option<Expression<'static>> {
        let head = token_deque.pop_first();
    }

    prattParse(0)
}

fn main() {
    println!("{:#?}", tokenize("1 + 2 + 3"));
}
