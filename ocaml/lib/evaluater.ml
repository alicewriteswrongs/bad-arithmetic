open Base
open Parser
open Tokenizer

let evaluate expr =
  let rec eval expr =
    match expr with
    | Num int -> int
    | Binop (left_expr, op, right_expr) -> (
        let left = eval left_expr in
        let right = eval right_expr in
        match op with
        | Add -> left + right
        | Subtract -> left - right
        | Multiply -> left * right
        | Divide -> left / right
        | Exponentiate -> Int.pow left right)
  in
  eval expr
