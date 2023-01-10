#[derive(Clone, Debug, PartialEq)]
pub struct SqlxValues(pub sea_query::Values);

#[cfg(any(
    feature = "sqlx-mysql",
    feature = "sqlx-postgres"
))]
pub struct EnumValue {
    pub name: Box<String>,
    pub value: Option<Box<String>>,
}
