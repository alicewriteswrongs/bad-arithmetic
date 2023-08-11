# 🐫

Do I know what I'm doing in OCaml? Not in the slightest!

This is what I do to run the code here:

```sh
eval $(opam env --set-switch)
opam install core core_bench utop ppx_jane
dune build
```

then you can open the ocaml top-level:

```sh
dune utop
```

and do something like

```ocaml
open Pratt.Tokenizer
open Pratt.Parser
open Pratt.Evaluater
```

to load the code into the REPL. Then you can do something like:


```ocaml
"1 + 2 * 3" |> tokenize |> parse |> Base.Option.map ~f:evaluate
```

neato!

You can also run a little binary with

```sh
./_build/default/bin/main.exe
```
