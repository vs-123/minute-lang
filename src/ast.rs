#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    String(String),
    // FunctionCall -> Name, Arguments
    FunctionCall(String, Vec<Node>)
}