```ebnf
Expr ::=
    Factor
  | Expr '+' Factor
Factor ::=
    Atom
  | Factor '*' Atom
Atom ::=
    'number'
  | '(' Expr ')'
```

| op              | priority |
| :-------------- | :------- |
| '.'             | (14, 13) |
| '!' , '['       | (11, ()) |
| Unary '+' , '-' | ((), 9)  |
| '\*' , '/'      | (7, 8)   |
| '+' , '-'       | (5, 6)   |
| '?'             | (4, 3)   |
| '='             | (2, 1)   |
