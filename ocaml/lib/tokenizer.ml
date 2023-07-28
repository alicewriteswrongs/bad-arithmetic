open Base

type operator = Add | Subtract | Multiply | Divide | Exponentiate
type token = TokenNum of int | TokenOp of operator

let to_op s =
  match s with
  | "+" -> Some (TokenOp Add)
  | "-" -> Some (TokenOp Subtract)
  | "*" -> Some (TokenOp Multiply)
  | "/" -> Some (TokenOp Divide)
  | "^" -> Some (TokenOp Exponentiate)
  | _ -> None

let tokenize s =
  let int_of_string_opt s =
    try Some (Int.of_string s) with Failure _ -> None
  in
  let f s =
    match int_of_string_opt s with
    | Some int -> Some (TokenNum int)
    | None -> to_op s
  in
  s
  |> String.split ~on:' '
  |> List.filter ~f:(fun s -> not (String.is_empty s))
  |> List.map ~f
  |> Option.all
