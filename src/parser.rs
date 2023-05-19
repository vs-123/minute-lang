use crate::{tokens::Token, ast::Node};

pub struct Parser{
    pub output_nodes: Vec<Node>,

    input_tokens: Vec<Token>,
    input_tokens_length: usize,
    current_token_index: usize,
}

impl Parser{
    pub fn new(input_tokens: Vec<Token>) -> Self{
        Self {
            input_tokens_length: input_tokens.len(),
            input_tokens,
            current_token_index: 0,

            output_nodes: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while self.is_not_last_token() {
            self.next();
        }
    }

    fn next(&mut self) {
        self.current_token_index += 1;
    }

    #[inline]
    fn current_token(&self) -> Token {
        self.input_tokens[self.current_token_index].clone()
    }

    #[inline]
    fn is_last_token(&self) -> bool {
        self.current_token_index >= self.input_tokens_length
    }

    #[inline]
    fn is_not_last_token(&self) -> bool {
        self.current_token_index < self.input_tokens_length
    }
}