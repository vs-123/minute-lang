use crate::{tokens::Token, ast::Node};

pub struct Parser{
    pub output_nodes: Vec<Node>,

    input_tokens: Vec<Token>,
    current_token_index: usize,
}

impl Parser{
    pub fn new(input_tokens: Vec<Token>) -> Self{
        Self {
            input_tokens,
            current_token_index: 0,

            output_nodes: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        todo!();
    }

    fn next(&self) {
        todo!();
    }
}