#[derive(Debug, Clone)]
pub struct CoqArgument {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct CoqDefinition {
    pub name: String,
    pub args: Vec<CoqArgument>,
    pub return_type: String,
    pub body: String,
}
