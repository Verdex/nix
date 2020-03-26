

#[derive(Debug)]
pub struct Module {
    pub type_statements : Vec<Type>,
    pub fun_statements : Vec<Fun>,
    pub use_statements : Vec<Use>,
}

#[derive(Debug)]
pub enum Type {

}

#[derive(Debug)]
pub enum Fun {

}

#[derive(Debug)]
pub struct Use {
    pub module_name : Vec<Symbol>,
    pub imports : Vec<Import>,
}

#[derive(Debug)]
pub struct Symbol {
    pub value : String,
}

#[derive(Debug)]
pub enum Import {
    Everything,
    Single(String),
}
