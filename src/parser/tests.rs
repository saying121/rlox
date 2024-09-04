use std::sync::Arc;

use pretty_assertions::assert_eq;

use super::Parser;
use crate::{
    expr::{Binary, Exprs, Literal, LiteralType},
    tokens::{Token, TokenInner},
};

#[test]
fn test_equal() {
    let source: Arc<str> = Arc::from("1==1");

    let tks = vec![
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 0),
        },
        Token::EqualEqual {
            inner: TokenInner::new(Arc::clone(&source), "==".to_owned(), 1),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 3),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::EqualEqual {
            inner: TokenInner::new(source, "==".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    // ///

    let source: Arc<str> = Arc::from("1!=1");

    let tks = vec![
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 0),
        },
        Token::BangEqual {
            inner: TokenInner::new(Arc::clone(&source), "!=".to_owned(), 1),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 3),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::BangEqual {
            inner: TokenInner::new(source, "!=".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);
}

#[test]
fn test_precedence() {
    let source: Arc<str> = Arc::from("6/3+16/2");

    let tks = vec![
        Token::Number {
            double: 6.0,
            inner: TokenInner::new(Arc::clone(&source), "6".to_owned(), 0),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 1),
        },
        Token::Number {
            double: 3.0,
            inner: TokenInner::new(Arc::clone(&source), "3".to_owned(), 2),
        },
        Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "+".to_owned(), 3),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "16".to_owned(), 4),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 5),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "2".to_owned(), 6),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(6.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 1),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(3.0),
            })),
        })),
        operator: Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "+".to_owned(), 3),
        },
        right: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(16.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 5),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(2.0),
            })),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    let source: Arc<str> = Arc::from("6/3-16*2");

    let tks = vec![
        Token::Number {
            double: 6.0,
            inner: TokenInner::new(Arc::clone(&source), "6".to_owned(), 0),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 1),
        },
        Token::Number {
            double: 3.0,
            inner: TokenInner::new(Arc::clone(&source), "3".to_owned(), 2),
        },
        Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "-".to_owned(), 3),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "16".to_owned(), 4),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "*".to_owned(), 5),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "2".to_owned(), 6),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(6.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 1),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(3.0),
            })),
        })),
        operator: Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "-".to_owned(), 3),
        },
        right: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(16.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new(Arc::clone(&source), "*".to_owned(), 5),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(2.0),
            })),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    let source: Arc<str> = Arc::from("6/3-16*-2");

    let tks = vec![
        Token::Number {
            double: 6.0,
            inner: TokenInner::new(Arc::clone(&source), "6".to_owned(), 0),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 1),
        },
        Token::Number {
            double: 3.0,
            inner: TokenInner::new(Arc::clone(&source), "3".to_owned(), 2),
        },
        Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "-".to_owned(), 3),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "16".to_owned(), 4),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "*".to_owned(), 5),
        },
        Token::Number {
            double: -2.0,
            inner: TokenInner::new(Arc::clone(&source), "-2".to_owned(), 6),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(6.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 1),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(3.0),
            })),
        })),
        operator: Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "-".to_owned(), 3),
        },
        right: Box::new(Exprs::Binary(Binary {
            left: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(16.0),
            })),
            operator: Token::Slash {
                inner: TokenInner::new(Arc::clone(&source), "*".to_owned(), 5),
            },
            right: Box::new(Exprs::Literal(Literal {
                value: LiteralType::Number(-2.0),
            })),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);
}

#[test]
fn test_plus_minus_multi_div() {
    // error end
    let source: Arc<str> = Arc::from("1+");

    let tks = vec![
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 0),
        },
        Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "+".to_owned(), 1),
        },
    ];

    let mut p = Parser::new(tks);
    match p.parse() {
        Ok(_) => unreachable!("It's invalid"),
        Err(e) => assert_eq!("End of source code, no next token.", e.to_string()),
    }

    // plus
    let source: Arc<str> = Arc::from("1+1");

    let tks = vec![
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 0),
        },
        Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "+".to_owned(), 1),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 2),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::Plus {
            inner: TokenInner::new(source, "+".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    // plus strings
    let source: Arc<str> = Arc::from(r#""ab"+"cd""#);

    let tks = vec![
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), "ab".to_owned(), 0),
        },
        Token::Plus {
            inner: TokenInner::new(Arc::clone(&source), "+".to_owned(), 4),
        },
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), "cd".to_owned(), 5),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::String("ab".to_owned()),
        })),
        operator: Token::Plus {
            inner: TokenInner::new(source, "+".to_owned(), 4),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::String("cd".to_owned()),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    // minus
    let source: Arc<str> = Arc::from("1-1");

    let tks = vec![
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 0),
        },
        Token::Minus {
            inner: TokenInner::new(Arc::clone(&source), "-".to_owned(), 1),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 2),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::Minus {
            inner: TokenInner::new(source, "-".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    // multiplication
    let source: Arc<str> = Arc::from("1*1");

    let tks = vec![
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 0),
        },
        Token::Star {
            inner: TokenInner::new(Arc::clone(&source), "*".to_owned(), 1),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 2),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::Star {
            inner: TokenInner::new(source, "*".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);

    // div
    let source: Arc<str> = Arc::from("1/1");

    let tks = vec![
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 0),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 1),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 2),
        },
    ];
    let right = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::Slash {
            inner: TokenInner::new(source, "/".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });

    let mut p = Parser::new(tks);
    let exprs = p.parse().unwrap();
    assert_eq!(right, exprs);
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
        Err(e) => assert_eq!("End of source code, no next token.", e.to_string()),
    }
}
