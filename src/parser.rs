use crate::{tokens::{Token, TokenKind}, ast::Node};

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
            let current_token = self.current_token();
            match current_token.kind {
                TokenKind::Identifier => {
                    
                }

                other => {
                    self.throw_err(format!(
                        "Unimplemented token kind '{:?}'",
                        other,
                    ))
                },
            }

            self.next();
        }
    }

    fn next(&mut self) {
        self.current_token_index += 1;
    }

    fn peek(&self) -> Option<Token> {
        if self.is_last_token() { return None; }
        
        Some(self.input_tokens[self.current_token_index + 1].clone())
    }

    fn expect_next() {

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

    fn throw_err<T: Into<String>>(&self, msg: T) {
        let current_token_location = self.current_token().location;
        let start_line_number = current_token_location.start_line;
        let end_line_number = current_token_location.end_line;

        let start_col_number = current_token_location.start_col;
        let end_col_number = current_token_location.end_col;

        let line_number_spaces = " ".repeat(start_line_number.to_string().len());

        // Assuming for now that start_line_number == end_line_number

        println!("[Error]");
        println!("{}\n", msg.into());
        println!("[Location] {}:{}:{}", current_token_location.file_path, start_line_number, start_col_number);
        println!(" {} |", line_number_spaces);
        println!(
            " {} | {}",
            start_line_number,
            current_token_location.line,
        );
        println!(" {} |{}{}", line_number_spaces, " ".repeat(start_col_number), "^".repeat(end_col_number-start_col_number+1));

        std::process::exit(1);
    }
}