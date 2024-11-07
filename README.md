# Some nightly features

- [let-chains](https://github.com/rust-lang/rfcs/blob/master/text/2497-if-let-chains.md) so wonderful.
- [try-expr](https://github.com/rust-lang/rfcs/blob/master/text/2388-try-expr.mdrl)

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

```ebnf
program        → declaration* EOF ;

declaration    → funDecl
               | varDecl
               | statement ;

statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | returnStmt
               | whileStmt
               | block ;

returnStmt      → "return" expression? ";" ;

breakStmt      → "break" ";" ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression ")" statement ( breakStmt ) ;

whileStmt      → "while" "(" expression ")" statement ( breakStmt ) ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;

block          → "{" declaration* "}" ;

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

unary          → ( "!" | "-" ) unary | call ;

call           → primary ( "(" arguments? ")" )* ;

arguments      → expression ( "," expression )* ;

primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")"
               | IDENTiFIER;

varDecl        → "var" IDENTiFIER ( "=" expression )? ";" ;

funDecl        → "fun" function ;

function       → IDENTiFIER "(" parameters? ")" block ;
```

## Desugaring

```lox
for (var i = 0; i < 10; i = i + 1) print 1;
```

⇓

```lox
{
    var i = 0;
    while (i < 10) {
        print i;
        i = i + 1;
    }
}
```
