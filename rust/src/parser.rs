use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use crate::tokenizer::{Operation, Token};

fn precedence(op: &Operation) -> u8 {
    match op {
        Operation::Add => 1,
        Operation::Subtract => 1,
        Operation::Multiply => 2,
        Operation::Divide => 2,
        Operation::Exponentiate => 2,
    }
}



#[derive(Clone)]
enum Expression<'a> {
    BinOp(&'a Expression<'a>, Operation, &'a Expression<'a>),
    Num(u32),
}

fn pratt_parse(
    current_precedence: u8,
    tokens: &RefCell<VecDeque<Token>>,
) -> Option<Expression<'static>> {
    match tokens.borrow_mut().pop_front()? {
        Token::Op(_) => None,
        Token::Num(num) => {
            let left = Cell::new(Expression::Num(num));
            while tokens.borrow().len() != 0 {
                match tokens.borrow_mut().get(0).unwrap() {
                    Token::Op(op) => {
                        let prec = precedence(op);
                        if prec <= current_precedence {
                            let unwrapped = left.into_inner();
                            return Some(unwrapped);
                        }
                        tokens.borrow_mut().pop_front();
                        let right = pratt_parse(prec, tokens)?;
                        let unwrapped = left.into_inner();
                        left.replace(
                            Expression::BinOp(&unwrapped, *op, &right.clone())
                        );
                    }
                    Token::Num(_) => {
                        let unwrapped = left.into_inner();
                        return Some(unwrapped);
                    }
                }
            }
            let unwrapped = left.into_inner();
            Some(unwrapped)
        }
    }
}

fn parse(tokens: Vec<Token>) -> Option<Expression<'static>> {
    if tokens.len() == 0 {
        return None;
    }

    let token_deque = RefCell::new(VecDeque::from(tokens));

    return pratt_parse(0, &token_deque);
}
