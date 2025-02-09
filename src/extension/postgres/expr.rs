use super::PgBinOper;
use crate::{expr::private::Expression, Expr, IntoLikeExpr, SimpleExpr};

pub trait PgExpr: Expression {
    /// Express an postgres concatenate (`||`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Font::Name, Font::Variant, Font::Language])
    ///     .from(Font::Table)
    ///     .and_where(
    ///         Expr::val("a")
    ///             .concatenate("b")
    ///             .concat("c")
    ///             .concat("d"),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a' || 'b' || 'c' || 'd'"#
    /// );
    /// ```
    fn concatenate<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.bin_op(PgBinOper::Concatenate, right)
    }
    /// Alias of [`PgExpr::concatenate`]
    fn concat<T>(self, right: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.concatenate(right)
    }

    /// Express an postgres fulltext search matches (`@@`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Font::Name, Font::Variant, Font::Language])
    ///     .from(Font::Table)
    ///     .and_where(Expr::val("a & b").matches("a b"))
    ///     .and_where(Expr::col(Font::Name).matches("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' @@ 'a b' AND "name" @@ 'a b'"#
    /// );
    /// ```
    fn matches<T>(self, expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.bin_op(PgBinOper::Matches, expr)
    }

    /// Express an postgres fulltext search contains (`@>`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Font::Name, Font::Variant, Font::Language])
    ///     .from(Font::Table)
    ///     .and_where(Expr::val("a & b").contains("a b"))
    ///     .and_where(Expr::col(Font::Name).contains("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' @> 'a b' AND "name" @> 'a b'"#
    /// );
    /// ```
    fn contains<T>(self, expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.bin_op(PgBinOper::Contains, expr)
    }

    /// Express an postgres fulltext search contained (`<@`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Font::Name, Font::Variant, Font::Language])
    ///     .from(Font::Table)
    ///     .and_where(Expr::val("a & b").contained("a b"))
    ///     .and_where(Expr::col(Font::Name).contained("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' <@ 'a b' AND "name" <@ 'a b'"#
    /// );
    /// ```
    fn contained<T>(self, expr: T) -> SimpleExpr
    where
        T: Into<SimpleExpr>,
    {
        self.bin_op(PgBinOper::Contained, expr)
    }

    /// Express a `LIKE` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::tbl(Char::Table, Char::Character).ilike("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" ILIKE E'Ours\'%'"#
    /// );
    /// ```
    fn ilike<L>(self, like: L) -> SimpleExpr
    where
        L: IntoLikeExpr,
    {
        self.like_like(PgBinOper::ILike, like.into_like_expr())
    }

    /// Express a `NOT ILIKE` expression
    fn not_ilike<L>(self, like: L) -> SimpleExpr
    where
        L: IntoLikeExpr,
    {
        self.like_like(PgBinOper::NotILike, like.into_like_expr())
    }
}

impl PgExpr for Expr {}

impl PgExpr for SimpleExpr {}
