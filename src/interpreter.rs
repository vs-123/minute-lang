use std::io::{self, Write};

use crate::{
    ast::{Node, NodeKind},
    tokens::{Location},
};

#[derive(Clone)]
pub struct Interpreter {
    input_nodes: Vec<Node>,
}

impl Interpreter {
    pub fn new(input_nodes: Vec<Node>) -> Self {
        Self { input_nodes }
    }

    pub fn interpret(&mut self) {
        for node in self.clone().input_nodes.iter() {
            self.match_node(node.clone());
        }
    }

    fn match_node(&mut self, node: Node) {
        let node = node.clone();
        let node_kind = node.kind.clone();
        let node_location = node.location.clone();

        match node_kind {
            NodeKind::FunctionCall(function_name, arguments) => match function_name.as_str() {
                "print" => {
                    for argument in arguments.iter() {
                        match &argument.kind {
                            NodeKind::String(argument) => {
                                let mut stdout = io::stdout();
                                stdout.write(argument.as_bytes()).and(stdout.flush()).ok();
                            }

                            _ => {
                                self.throw_err(format!(
                                        "Invalid argument of kind '{:?}' for function '{}', expected of kind 'String'",
                                        argument.kind, function_name
                                    ), node_location.clone());
                            }
                        }
                    }
                }

                "syscall" => {
                    if arguments.len() < 1 {
                        self.throw_err(format!(
                            "Insufficient amount of arguments for function '{}', at least 1 required.\n[Help]\n{}",
                            function_name,
                            "([command_name], [arguments]...)\nAll the arguments are strings.",
                        ), node_location.clone());
                    }

                    let mut command_list = Vec::<String>::new();

                    for argument in arguments.iter() {
                        match &argument.kind {
                            NodeKind::String(argument) => {
                                command_list.push(argument.to_string());
                            }

                            _ => {
                                self.throw_err(format!(
                                        "Invalid argument of kind '{:?}' for function '{}', expected of kind 'String'",
                                        argument.kind, function_name
                                    ), node_location.clone());
                            }
                        }
                    }

                    let mut process_command = std::process::Command::new(command_list[0].clone());
                    for command in command_list.iter().skip(1) {
                        process_command.arg(command);
                    }

                    match process_command.output() {
                        Ok(ok) => {
                            let mut stdout = io::stdout();
                            stdout.write(&ok.stdout).and(stdout.flush()).ok();

                            if !ok.stderr.is_empty() {
                                stdout.write(&ok.stderr).and(stdout.flush()).ok();
                            }
                        }

                        Err(err) => {
                            self.throw_err(format!(
                                "Could not execute command.\nReason: {}",
                                err
                            ), node_location.clone());
                        }
                    }
                }

                other => {
                    self.throw_err(format!("Invalid function '{}'", other), node_location);
                }
            },

            other => {
                self.throw_err(format!("Unimplemented node '{:?}'", other), node_location);
            }
        }
    }

    fn throw_err<T: Into<String>>(&self, msg: T, node_location: Location) {
        let line_number_spaces = " ".repeat(node_location.start_line.to_string().len());

        println!("[Error]");
        println!("{}\n", msg.into());
        println!(
            "[Location] {}:{}:{}",
            node_location.file_path, node_location.start_line, node_location.start_col
        );
        println!(" {} |", line_number_spaces);
        println!(" {} | {}", node_location.start_line, node_location.line);
        println!(
            " {} | {}",
            line_number_spaces,
            "^".repeat(node_location.line.len())
        );
        std::process::exit(1);
    }
}
