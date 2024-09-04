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
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
```
