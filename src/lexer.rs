use crate::tokens::{Location, Token, TokenKind};

pub struct Lexer {
    pub output_tokens: Vec<Token>,

    source_code_length: usize,
    source_code_chars: Vec<char>,
    source_code_lines: Vec<String>,

    current_char_index: usize,
    line_start_indices: Vec<usize>,
    file_path: String,
}

impl Lexer {
    pub fn new(source_code: String, file_path: String) -> Self {
        Self {
            output_tokens: Vec::new(),

            source_code_length: source_code.len(),
            source_code_chars: source_code.chars().collect(),
            source_code_lines: source_code.split("\n").map(String::from).collect(),

            current_char_index: 0,
            line_start_indices: vec![0],
            file_path,
        }
    }

    pub fn lex(&mut self) {
        while self.is_not_eof() {
            // Comments
            if self.current_line().starts_with("//") { self.next(); continue; }
            match self.current_char() {
                c if c.is_whitespace() => {}

                c if c.is_alphabetic() => {
                    self.eat_identifier();
                }

                '"' => {
                    self.eat_string();
                }

                '(' => {
                    let current_col = self.current_col();
                    let current_line_number = self.current_line_number();

                    self.output_tokens.push(Token {
                        kind: TokenKind::OParen,
                        value: String::from("("),
                        location: Location {
                            start_col: current_col,
                            start_line: current_line_number,

                            end_col: current_col,
                            end_line: current_line_number,

                            file_path: self.file_path.clone(),
                            line: self.current_line(),
                        },
                    })
                }

                ')' => {
                    let current_col = self.current_col();
                    let current_line_number = self.current_line_number();

                    self.output_tokens.push(Token {
                        kind: TokenKind::CParen,
                        value: String::from(")"),
                        location: Location {
                            start_col: current_col,
                            start_line: current_line_number,

                            end_col: current_col,
                            end_line: current_line_number,

                            file_path: self.file_path.clone(),
                            line: self.current_line(),
                        },
                    })
                }

                ',' => {
                    let current_col = self.current_col();
                    let current_line_number = self.current_line_number();

                    self.output_tokens.push(Token {
                        kind: TokenKind::Comma,
                        value: String::from(","),
                        location: Location {
                            start_col: current_col,
                            start_line: current_line_number,

                            end_col: current_col,
                            end_line: current_line_number,

                            file_path: self.file_path.clone(),
                            line: self.current_line(),
                        },
                    })
                }

                ';' => {
                    let current_col = self.current_col();
                    let current_line_number = self.current_line_number();

                    self.output_tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        value: String::from(";"),
                        location: Location {
                            start_col: current_col,
                            start_line: current_line_number,

                            end_col: current_col,
                            end_line: current_line_number,

                            file_path: self.file_path.clone(),
                            line: self.current_line(),
                        },
                    })
                }

                other => self.throw_err(format!("Unexpected character '{}'", other)),
            }

            self.next();
        }
    }

    fn eat_identifier(&mut self) {
        let start_col = self.current_col();
        let start_line = self.current_line_number();

        let mut eaten_identifier = String::new();

        while self.current_char().is_alphanumeric() {
            eaten_identifier.push(self.current_char());
            if self.is_eof() {
                break;
            }
            self.current_char_index += 1;
        }

        self.current_char_index -= 1;

        self.output_tokens.push(Token {
            kind: TokenKind::Identifier,
            value: eaten_identifier,
            location: Location {
                start_col,
                start_line,

                end_col: self.current_col(),
                end_line: self.current_line_number(),

                file_path: self.file_path.clone(),
                line: self.current_line(),
            },
        })
    }

    fn eat_string(&mut self) {
        let start_col = self.current_col();
        let start_line = self.current_line_number();

        let mut eaten_string = String::new();
        self.current_char_index += 1;

        while self.current_char() != '"' {
            if self.is_eof() {
                self.throw_err(format!(
                    "Missing end of string '\"' since line {} at column {}",
                    start_line, start_col,
                ))
            }

            eaten_string.push(self.current_char());
            self.current_char_index += 1;
        }

        self.output_tokens.push(Token {
            kind: TokenKind::String,
            value: eaten_string,
            location: Location {
                start_col,
                start_line,

                end_col: self.current_col(),
                end_line: self.current_line_number(),

                file_path: self.file_path.clone(),
                line: self.current_line(),
            },
        })
    }

    fn next(&mut self) {
        self.current_char_index += 1;

        if self.is_not_eof() && self.current_char() == '\n' {
            self.line_start_indices.push(self.current_char_index);
            self.next();
        }
    }

    #[inline]
    fn is_not_eof(&self) -> bool {
        self.current_char_index < self.source_code_length
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.current_char_index + 1 >= self.source_code_length
    }

    #[inline]
    fn current_char(&mut self) -> char {
        self.source_code_chars[self.current_char_index].clone()
    }

    #[inline]
    fn current_col(&self) -> usize {
        self.current_char_index - self.line_start_indices.last().unwrap() + 1
    }

    #[inline]
    fn current_line_number(&self) -> usize {
        self.line_start_indices.len()
    }

    #[inline]
    fn current_line(&self) -> String {
        self.source_code_lines[self.current_line_number() - 1].clone()
    }

    fn throw_err<T: Into<String>>(&self, msg: T) {
        let current_line_number = self.current_line_number();
        let current_line_number_spaces = " ".repeat(current_line_number.to_string().len());
        let current_col = self.current_col();
        let arrow_spaces = " ".repeat(current_col - 1);

        println!("[Error]");
        println!("{}\n", msg.into());
        println!(
            "[Location] {}:{}:{}",
            self.file_path, current_line_number, current_col
        );
        println!(" {} |", current_line_number_spaces);
        println!(" {} | {}", current_line_number, self.current_line(),);
        println!(" {} | {}^", current_line_number_spaces, arrow_spaces);

        std::process::exit(1);
    }
}
