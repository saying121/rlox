[let-chains](https://github.com/rust-lang/rfcs/blob/master/text/2497-if-let-chains.md) so wonderful.

## precedence and associativity

| Name       | Operators            | Associates |
| ---------- | -------------------- | ---------- |
| Logic      | `and`                | Left       |
| Logic      | `or`                 | Left       |
| Equality   | `==`, `!=`           | Left       |
| Comparison | `>`, `>=`, `<`, `<=` | Left       |
| Term       | `-`, `+`             | Left       |
| Factor     | `*`, `/`             | Left       |
| Unary      | `!`, `-`             | Right      |

```bng
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

statement      → exprStmt
               | ifStmt
               | printStmt
               | block ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;

block          → "{" declaration"}" ;

exprStmt       → expression ";" ;

printStmt      → "print" expression ";" ;

expression     → assignment ;

assignment     → IDENTiFIER "=" assignment
               | logic_or ;

logic_or       → logic_and ( "or" logic_and )* ;

logic_and      → equality ( "and" equality )* ;

equality       → comparison ( ( "!=" | "==" ) comparison )* ;

comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term           → factor ( ( "-" | "+" ) factor )* ;

factor         → unary ( ( "/" | "*" ) unary )* ;

unary          → ( "!" | "-" ) unary
               | primary ;

primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")"
               | IDENTiFIER;
```

```bng
varDecl        → "var" IDENTiFIER ( "=" expression )? ";" ;
```
