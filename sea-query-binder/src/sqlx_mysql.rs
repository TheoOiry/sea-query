use crate::values::EnumValue;
use crate::SqlxValues;
use sea_query::Value;
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::Database;
use sqlx::MySql;

impl<'q> sqlx::Encode<'q, MySql> for EnumValue {
    fn encode(self, buf: &mut <MySql as HasArguments<'q>>::ArgumentBuffer) -> IsNull
    where
        Self: Sized,
    {
        let v = self.value.map(|v| *v);
        <Option<String> as sqlx::Encode<MySql>>::encode(v, buf)
    }

    fn encode_by_ref(&self, buf: &mut <MySql as HasArguments>::ArgumentBuffer) -> IsNull {
        let v = self.value.as_deref();
        <Option<&String> as sqlx::Encode<MySql>>::encode_by_ref(&v, buf)
    }
}

impl sqlx::Type<MySql> for EnumValue {
    fn type_info() -> <MySql as Database>::TypeInfo {
        <MySql as Database>::TypeInfo::__enum()
    }
}

impl<'q> sqlx::IntoArguments<'q, sqlx::mysql::MySql> for SqlxValues {
    fn into_arguments(self) -> sqlx::mysql::MySqlArguments {
        let mut args = sqlx::mysql::MySqlArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    args.add(b);
                }
                Value::TinyInt(i) => {
                    args.add(i);
                }
                Value::SmallInt(i) => {
                    args.add(i);
                }
                Value::Int(i) => {
                    args.add(i);
                }
                Value::BigInt(i) => {
                    args.add(i);
                }
                Value::TinyUnsigned(i) => {
                    args.add(i);
                }
                Value::SmallUnsigned(i) => {
                    args.add(i);
                }
                Value::Unsigned(i) => {
                    args.add(i);
                }
                Value::BigUnsigned(i) => {
                    args.add(i);
                }
                Value::Float(f) => {
                    args.add(f);
                }
                Value::Double(d) => {
                    args.add(d);
                }
                Value::String(s) => {
                    args.add(s.as_deref());
                }
                Value::Char(c) => {
                    args.add(c.map(|c| c.to_string()));
                }
                Value::Bytes(b) => {
                    args.add(b.as_deref());
                }
                Value::Enum(.., value) => args.add(EnumValue {
                    postgres_oid: 0,
                    value,
                }),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    args.add(d.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    args.add(Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    args.add(t.as_deref());
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    args.add(uuid.map(|v| v.to_string()));
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(d) => {
                    args.add(d.as_deref());
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(d) => {
                    args.add(d.as_deref());
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    args.add(j.as_deref());
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    panic!("Mysql doesn't support array arguments");
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(i) => args.add(i.map(|i| i.to_string())),
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(m) => args.add(m.map(|m| m.to_string())),
            }
        }
        args
    }
}
