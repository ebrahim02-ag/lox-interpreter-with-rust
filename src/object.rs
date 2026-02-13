
//an enume to emulate Java's Object type
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Boolean(bool),
    Null,
    Number(f64),
    String(String)
}
