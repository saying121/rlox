use rlox::token::Token;

use super::{
    Parser, Precedence,
    state::{CompileState, Compiling},
};
use crate::error::Result;

pub type ParseFn<I, S> = for<'a> fn(&'a mut Parser<I, S>, bool) -> Result<()>;

pub struct ParseRule<I, S>
where
    I: Iterator<Item = Token>,
    S: CompileState,
{
    pub prefix: Option<ParseFn<I, S>>,
    pub infix: Option<ParseFn<I, S>>,
    pub precedence: Precedence,
}

#[expect(clippy::match_same_arms, reason = "align")]
pub fn get_rule<I>(typ: &Token) -> ParseRule<I, Compiling>
where
    I: Iterator<Item = Token>,
{
    match typ {
        Token::LeftParen { .. } => ParseRule {
            prefix: Some(Parser::grouping),
            infix: None,
            precedence: Precedence::None,
        },
        Token::RightParen { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::LeftBrace { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::RightBrace { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Comma { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Dot { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Minus { .. } => ParseRule {
            prefix: Some(Parser::unary),
            infix: Some(Parser::binary),
            precedence: Precedence::Term,
        },
        Token::Plus { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Term,
        },
        Token::Semicolon { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Slash { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Factor,
        },
        Token::Star { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Factor,
        },
        Token::Bang { .. } => ParseRule {
            prefix: Some(Parser::unary),
            infix: None,
            precedence: Precedence::None,
        },
        Token::BangEqual { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Equality,
        },
        Token::Equal { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::EqualEqual { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Equality,
        },
        Token::Greater { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
        Token::GreaterEqual { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
        Token::Less { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
        Token::LessEqual { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
        Token::Identifier { .. } => ParseRule {
            prefix: Some(Parser::variable),
            infix: None,
            precedence: Precedence::None,
        },
        Token::String { .. } => ParseRule {
            prefix: Some(Parser::string),
            infix: None,
            precedence: Precedence::None,
        },
        Token::Number { .. } => ParseRule {
            prefix: Some(Parser::number),
            infix: None,
            precedence: Precedence::None,
        },
        Token::And { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::and),
            precedence: Precedence::And,
        },
        Token::Class { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Else { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Fun { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::For { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::If { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Nil { .. } => ParseRule {
            prefix: Some(Parser::literal),
            infix: None,
            precedence: Precedence::None,
        },
        Token::Or { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Print { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Return { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Super { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::This { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::True { .. } => ParseRule {
            prefix: Some(Parser::literal),
            infix: None,
            precedence: Precedence::None,
        },
        Token::False { .. } => ParseRule {
            prefix: Some(Parser::literal),
            infix: None,
            precedence: Precedence::None,
        },
        Token::Var { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::While { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Eof { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Comment { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::BlockComment { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Break { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Invalid { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    }
}
