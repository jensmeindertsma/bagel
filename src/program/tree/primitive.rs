#[derive(Debug)]
pub enum Primitive {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}
