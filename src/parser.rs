use crate::{
    ast::{Node, NodeKind},
    tokens::{Token, TokenKind},
};

pub struct Parser {
    pub output_nodes: Vec<Node>,

    input_tokens: Vec<Token>,
    input_tokens_length: usize,
    current_token_index: usize,
}

const NEXT_ARGUMENT_TOKENS: [TokenKind; 3] =
    [TokenKind::CParen, TokenKind::String, TokenKind::Comma];

impl Parser {
    pub fn new(input_tokens: Vec<Token>) -> Self {
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
                // This Identifier only refers to the function name
                // E.g. print()
                //      ^^^^^
                TokenKind::Identifier => {
                    let function_name = self.current_token().value;

                    self.expect_next(TokenKind::OParen);
                    self.next();
                    self.expect_next_either(&NEXT_ARGUMENT_TOKENS);
                    self.next();

                    // Arguments

                    let mut arguments = Vec::<Node>::new();

                    loop {
                        let current_token = self.current_token();

                        match current_token.kind {
                            TokenKind::CParen => break,

                            TokenKind::String => {
                                let new_value = self.current_token().value.replace("\\n", "\n");

                                arguments.push(Node {
                                    kind: NodeKind::String(new_value),
                                    location: current_token.location,
                                });
                            }

                            TokenKind::Comma => {}

                            other => self
                                .throw_err(format!("Unexpected token {:?} for arguments", other)),
                        }

                        self.expect_next_either(&NEXT_ARGUMENT_TOKENS);
                        self.next();
                    }

                    self.expect_next(TokenKind::Semicolon);
                    self.next();

                    self.output_nodes.push(Node {
                        kind: NodeKind::FunctionCall(function_name, arguments),
                        location: self.current_token().location,
                    });
                }

                other => self.throw_err(format!("Unexpected token kind '{:?}'", other,)),
            }

            self.next();
        }
    }

    fn next(&mut self) {
        self.current_token_index += 1;
    }

    fn peek(&self) -> Option<Token> {
        if self.current_token_index + 1 >= self.input_tokens_length {
            return None;
        }

        Some(self.input_tokens[self.current_token_index + 1].clone())
    }

    fn expect_next(&mut self, expected_kind: TokenKind) {
        let next_token = self.peek();
        let current_token = self.current_token();

        if next_token.is_none() {
            self.throw_err(format!(
                "Expected token after '{}' to be of kind '{:?}', but is end of file.",
                current_token.value, expected_kind
            ));
        }

        let next_token = next_token.unwrap();

        if next_token.kind != expected_kind {
            self.input_tokens[self.current_token_index]
                .location
                .start_col = self.input_tokens[self.current_token_index].location.end_col;

            self.throw_err(format!(
                "Expected token after '{}' to be of kind '{:?}', but found '{}' which is of kind '{:?}'",
                current_token.value, expected_kind, next_token.value, next_token.kind
            ));
        }
    }

    fn expect_next_either(&mut self, expected_kinds: &[TokenKind]) {
        let next_token = self.peek();
        let current_token = self.current_token();

        if next_token.is_none() {
            self.throw_err(format!(
                "Expected token after '{}' to be either of kinds '{:?}', but is end of file.",
                current_token.value, expected_kinds
            ));
        }

        let next_token = next_token.unwrap();

        if !expected_kinds.contains(&next_token.kind) {
            self.throw_err(format!(
                "Expected token after '{}' to be either of kinds '{:?}', but found '{}' which is of kind '{:?}'.",
                current_token.value, expected_kinds, next_token.value, next_token.kind
            ));
        }
    }

    #[inline]
    fn current_token(&self) -> Token {
        self.input_tokens[self.current_token_index].clone()
    }

    #[inline]
    fn is_not_last_token(&self) -> bool {
        self.current_token_index < self.input_tokens_length
    }

    fn throw_err<T: Into<String>>(&self, msg: T) {
        let current_token_location = self.current_token().location;
        let start_line_number = current_token_location.start_line;
        // let end_line_number = current_token_location.end_line;

        let start_col_number = current_token_location.start_col;
        let end_col_number = current_token_location.end_col;

        let line_number_spaces = " ".repeat(start_line_number.to_string().len());

        // Assuming for now that start_line_number == end_line_number

        println!("[Error]");
        println!("{}\n", msg.into());
        println!(
            "[Location] {}:{}:{}",
            current_token_location.file_path, start_line_number, start_col_number
        );
        println!(" {} |", line_number_spaces);
        println!(" {} | {}", start_line_number, current_token_location.line,);
        println!(
            " {} |{}{}",
            line_number_spaces,
            " ".repeat(start_col_number),
            "^".repeat(end_col_number - start_col_number + 1)
        );

        std::process::exit(1);
    }
}
