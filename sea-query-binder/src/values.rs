#[derive(Clone, Debug, PartialEq)]
pub struct SqlxValues(pub sea_query::Values);

pub struct EnumValue {
    pub name: Box<String>,
    pub value: Option<Box<String>>,
}
