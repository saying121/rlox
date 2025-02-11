use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Atom(char),
    Op(char),
    Eof,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut tokens: Vec<_> = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect();
        tokens.reverse();
        Self { tokens }
    }

    #[expect(clippy::should_implement_trait, reason = "")]
    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum S {
    Atom(char),
    Cons(char, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(i) => write!(f, "{}", i),
            Self::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?;
                }
                write!(f, ")",)
            }
        }
    }
}

pub fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

pub fn expr_bp(lexer: &mut Lexer, cur_min: u8) -> S {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        Token::Op(op) => {
            let (_, r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp);
            S::Cons(op, vec![rhs])
        }
        t => panic!("Bad Token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("Bad Token: {:?}", t),
        };
        if let Some((l_bp, _)) = postfix_binding_power(op) {
            if l_bp < cur_min {
                break;
            }
            lexer.next();

            lhs = if op == '[' {
                let rhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(']'));
                S::Cons(op, vec![lhs, rhs])
            } else {
                S::Cons(op, vec![lhs])
            };

            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < cur_min {
                break;
            }

            lexer.next();
            lhs = if op == '?' {
                let mhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(':'));
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, mhs, rhs])
            } else {
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, rhs])
            };

            continue;
        }

        break;
    }

    lhs
}

const fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    match op {
        '!' | '[' => Some((11, ())),
        _ => None,
    }
}
fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '+' | '-' => ((), 9),
        _ => panic!("Bad op: {:?}", op),
    }
}

const fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    Some(match op {
        '=' => (2, 1),
        '?' => (4, 3),
        '+' | '-' => (5, 6),
        '*' | '/' => (7, 8),
        '.' => (14, 13),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr_work() {
        let s = expr("1");
        assert_eq!(s.to_string(), "1");
    }

    #[test]
    fn expr_work1() {
        let s = expr("1 + 2 * 3");
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
    }

    #[test]
    fn expr_work2() {
        let s = expr("a + b * c * d + e");
        assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");
    }

    #[test]
    fn prefix_work() {
        let s = expr("--1 * 2");
        assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

        let s = expr("--f . g");
        assert_eq!(s.to_string(), "(- (- (. f g)))");

        let s = expr("--1 * -2");
        assert_eq!(s.to_string(), "(* (- (- 1)) (- 2))");
    }

    #[test]
    fn postfix_work() {
        let s = expr("-9!");
        assert_eq!(s.to_string(), "(- (! 9))");

        let s = expr("f . g !");
        assert_eq!(s.to_string(), "(! (. f g))");
    }

    #[test]
    fn parens_work() {
        let s = expr("(((0)))");
        assert_eq!(s.to_string(), "0");
    }

    #[test]
    fn idx_work() {
        let s = expr("x[0][1]");
        assert_eq!(s.to_string(), "([ ([ x 0) 1)");
    }

    #[test]
    fn tri_work() {
        let s = expr(
            "a ? b :
         c ? d
         : e",
        );
        assert_eq!(s.to_string(), "(? a b (? c d e))");
        let s = expr("a = 0 ? b : c = d");
        assert_eq!(s.to_string(), "(= a (= (? 0 b c) d))");
    }
}
