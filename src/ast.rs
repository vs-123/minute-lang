#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    String(String),
    // FunctionCall -> Name, Optional Arguments
    FunctionCall(String, Option<Vec<Box<Node>>>)
}