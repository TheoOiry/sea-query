#[derive(Clone, Debug, PartialEq)]
pub struct SqlxValues(pub sea_query::Values);

pub struct EnumValue {
    pub postgres_oid: u32,
    pub value: Option<Box<String>>,
}