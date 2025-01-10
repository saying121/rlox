use std::collections::HashMap;

use pretty_assertions::assert_eq;

use super::LoxClass;

#[test]
fn class_display() {
    let lox_class = LoxClass {
        name: "test".to_owned(),
        methods: HashMap::new(),
        superclass: None,
    };
    assert_eq!(lox_class.to_string(), "test");
}
