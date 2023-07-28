open Base
open Tokenizer
open Base.Option

type expr = Num of int | Binop of expr * operator * expr

let precedence (op : operator) =
  match op with
  | Add -> 1
  | Subtract -> 1
  | Multiply -> 2
  | Divide -> 2
  | Exponentiate -> 3

let prefix tokens =
  match tokens with
  | [] -> None
  | TokenNum n :: rest -> Some (Num n, rest)
  | TokenOp _ :: _ -> None

let rec pratt precLimit tokens =
  match prefix tokens with
  | None -> None
  | Some (left, remainder) -> prattLoop precLimit left remainder

and prattLoop precLimit left remainder =
  match remainder with
  | TokenOp op :: tokensAfterOp ->
      let opPrec = precedence op in
      if opPrec > precLimit then
        match pratt opPrec tokensAfterOp with
        | Some (right, tokensAfterChild) ->
            let newLeft = Binop (left, op, right) in
            prattLoop precLimit newLeft tokensAfterChild
        | None -> None
      else Some (left, remainder)
  | _ -> Some (left, remainder)

let parse tokens =
  let f tokens =
    match pratt 0 tokens with
    | Some (x, _) -> Some x
    | None -> None
  in
  tokens >>= f
