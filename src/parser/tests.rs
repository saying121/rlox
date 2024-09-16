use std::sync::Arc;

use pretty_assertions::assert_eq;

use super::Parser;
use crate::{
    expr::{Binary, Exprs, Literal, LiteralType, Unary},
    scan::scanner,
    stmt::{Print, Stmts, Var},
    tokens::{Token, TokenInner},
};

#[test]
fn test_equal() {
    let source: Arc<str> = Arc::from("var a = 1==1;");
    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            }),
            Token::EqualEqual {
                inner: TokenInner::new(Arc::clone(&source), "==".to_owned(), 9),
            },
            Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            }),
        )),
    ))];

    let mut p = Parser::new(tks);
    let stmts = p.parse().unwrap();
    assert_eq!(right, stmts);
}

#[test]
fn test_precedence() {
    let source: Arc<str> = Arc::from("print 6/3+16/2;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary {
        left: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(6.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(3.0),
            })),
        })),
        operator: Token::Plus {
            inner: TokenInner::new_plus(Arc::clone(&source), 9),
        },
        right: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(16.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 12),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(2.0),
            })),
        })),
    })))];
    let mut p = Parser::new(tks);
    let stmts = p.parse().unwrap();
    assert_eq!(right, stmts);

    let source: Arc<str> = Arc::from("print 6/3-16*2;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary {
        left: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(6.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(3.0),
            })),
        })),
        operator: Token::Minus {
            inner: TokenInner::new_minus(Arc::clone(&source), 9),
        },
        right: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(16.0),
            })),
            operator: Token::Star {
                inner: TokenInner::new_star(Arc::clone(&source), 12),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(2.0),
            })),
        })),
    })))];

    let mut p = Parser::new(tks);
    let stmts = p.parse().unwrap();
    assert_eq!(right, stmts);

    let source: Arc<str> = Arc::from("print 6/3-16*-2;");
    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary {
        left: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(6.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(3.0),
            })),
        })),
        operator: Token::Minus {
            inner: TokenInner::new_minus(Arc::clone(&source), 9),
        },
        right: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(16.0),
            })),
            operator: Token::Star {
                inner: TokenInner::new_star(Arc::clone(&source), 12),
            },
            right: Box::new(Exprs::Unary(Unary {
                operator: Token::Minus {
                    inner: TokenInner::new_minus(Arc::clone(&source), 13),
                },
                right: Box::new(Exprs::Literal(Literal {
                    value: LiteralType::Number(2.0),
                })),
            })),
        })),
    })))];

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);
}

#[test]
fn test_plus_minus_multi_div() {
    // plus
    let source: Arc<str> = Arc::from("var a=1+1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
            operator: Token::Plus {
                inner: TokenInner::new_plus(Arc::clone(&source), 7),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
        }),
    ))];

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    // plus strings
    let source: Arc<str> = Arc::from(r#"var a="ab"+"cd";"#);

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::String("ab".to_owned()),
            })),
            operator: Token::Plus {
                inner: TokenInner::new_plus(Arc::clone(&source), 10),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::String("cd".to_owned()),
            })),
        }),
    ))];

    let mut p = Parser::new(tks);
    let stmts = p.parse().unwrap();
    assert_eq!(right, stmts);

    // minus
    let source: Arc<str> = Arc::from("var a=1-1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
            operator: Token::Minus {
                inner: TokenInner::new_minus(Arc::clone(&source), 7),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
        }),
    ))];

    let mut p = Parser::new(tks);
    let stmts = p.parse().unwrap();
    assert_eq!(right, stmts);

    // multiplication
    let source: Arc<str> = Arc::from("var a=1*1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
            operator: Token::Star {
                inner: TokenInner::new_star(Arc::clone(&source), 7),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
        }),
    ))];

    let mut p = Parser::new(tks);
    let stmts = p.parse().unwrap();
    assert_eq!(right, stmts);

    // div
    let source: Arc<str> = Arc::from("var a=1/1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(1.0),
            })),
        }),
    ))];

    let mut p = Parser::new(tks);
    let stmts = p.parse().unwrap();
    assert_eq!(right, stmts);
}

#[test]
fn test_paren() {
    let source: Arc<str> = Arc::from("(");

    let tks = vec![Token::LeftParen {
        inner: TokenInner::new(Arc::clone(&source), "(".to_owned(), 0),
    }];

    let mut p = Parser::new(tks);
    match p.parse() {
        Ok(_) => unreachable!("It's invalid"),
        Err(e) => assert_eq!("End of source code, no next token", e.to_string()),
    }
}
