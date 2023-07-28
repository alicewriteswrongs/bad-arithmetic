open Base
open Pratt.Parser
open Pratt.Tokenizer
open Pratt.Evaluater
open Stdio

let main =
  let to_log = "1 + 22 * 3 " |> tokenize |> parse |> Option.map ~f:evaluate in
  match to_log with
  | Some num -> num |> Int.to_string |> print_endline
  | None -> print_endline "didnt work"

let () = main
