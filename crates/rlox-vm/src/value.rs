#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct Value(f64);

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct ValueArray(pub Vec<Value>);

impl ValueArray {
    pub const fn new() -> Self {
        Self(vec![])
    }

    pub fn write(&mut self, v: Value) {
        self.0.push(v);
    }
}
