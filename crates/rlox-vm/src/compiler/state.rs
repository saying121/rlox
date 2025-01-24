use crate::chunk::Chunk;

pub trait CompileState {
    type Data;
}
#[derive(Clone)]
#[derive(Debug)]
pub struct Init;

#[derive(Clone)]
#[derive(Debug)]
pub struct Compiling;

impl CompileState for Init {
    type Data = ();
}

impl CompileState for Compiling {
    type Data = Chunk;
}
