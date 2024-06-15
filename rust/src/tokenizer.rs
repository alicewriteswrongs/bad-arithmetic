#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiate,
}

#[derive(Debug, Clone)]
pub enum Token {
    // TODO this should be an i32, but tokenizer needs to be more mature for that
    Num(u32),
    Op(Operation),
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

pub fn tokenize(code: &str) -> Option<Vec<Token>> {
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
