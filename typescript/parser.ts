const OPERATORS: Record<Operation, number> = {
  "+": 1,
  "-": 1,
  "*": 2,
  "/": 2,
  "^": 2,
};

type Operation = "+" | "-" | "*" | "/" | "^";

type Token = number | Operation;

const isOperation = (arg: string | Operation | number): arg is Operation =>
  Object.keys(OPERATORS).includes(arg as string);

const tokenize = (code: string): Token[] =>
  code
    .replace(/\s/g, "")
    .split("")
    .map((opOrNumber) => {
      let parsed = parseInt(opOrNumber, 10);
      if (!isNaN(parsed)) {
        return parsed;
      }
      if (isOperation(opOrNumber)) {
        return opOrNumber;
      }
      throw new Error(
        `Parsing error! Encountered an unparseable character: ${opOrNumber}`,
      );
    });

type BinaryOperation = {
  left: Expression;
  op: Operation;
  right: Expression;
};

type Expression = BinaryOperation | number;

const parse = (tokens: Token[]): Expression => {
  tokens = tokens.concat();
  if (tokens.length === 0) {
    throw new Error("Nothing to parse!");
  }

  const prattParse = (precedence: number): Expression => {
    const maybeToken = tokens.shift();
    if (maybeToken === undefined || typeof maybeToken !== "number") {
      throw new Error("I should have found a number here...");
    }

    let left: Expression = maybeToken;

    while (tokens[0] !== undefined && isOperation(tokens[0])) {
      // we know the next token is an operator so we
      // grab the precedence for it
      const prec = OPERATORS[tokens[0]];

      if (prec <= precedence) {
        return left;
      }

      const op = tokens.shift();
      if (typeof op === "number" || op === undefined) {
        throw new Error("I expected an operation here");
      }

      const right = prattParse(prec);
      left = { left, op, right };
    }
    return left;
  }

  return prattParse(0);
};

const evaluate = (expr: Expression): number => {
  if (typeof expr === "number") {
    return expr;
  }

  const left = evaluate(expr.left);
  const right = evaluate(expr.right);

  switch (expr.op) {
    case "+": return left + right;
    case "-": return left - right;
    case "*": return left * right;
    case "/": return left / right;
    case "^": return Math.pow(left, right);
  }
}

const run = (ops: string) => {
  const tokens = tokenize(ops);
  console.log('TOKENS', tokens);
  const expr = parse(tokens);
  console.log('EXPR', expr);
  const value = evaluate(expr);
  console.log('VALUE', value);
}

run("1 + 2 * 3 ")
