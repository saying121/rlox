use pretty_assertions::assert_eq;

use super::LoxClass;

#[test]
fn class_display() {
    let lox_class = LoxClass {
        name: "test".to_owned(),
    };
    assert_eq!(lox_class.to_string(), "test");
}
