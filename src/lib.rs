#![feature(let_chains)]

pub mod cli;
pub mod prompt;
pub mod tokens;
pub mod scan;
pub mod lox;


#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {
        let a  = &123;
        dbg!(a);
    }
}
