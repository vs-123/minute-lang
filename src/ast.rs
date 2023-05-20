use crate::tokens::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    String(String),
    // FunctionCall -> Name, Arguments
    FunctionCall(String, Vec<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub location: Location,
}
