[let-chains](https://github.com/rust-lang/rfcs/blob/master/text/2497-if-let-chains.md) so wonderful.

## precedence and associativity

| Name       | Operators            | Associates |
| ---------- | -------------------- | ---------- |
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
               | printStmt ;

exprStmt       → expression ";" ;

printStmt      → "print" expression ";" ;

expression     → equality ;

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
