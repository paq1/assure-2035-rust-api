use mongodb::bson::{doc, Document};

use crate::core::shared::repositories::filter::Expr::ExprStr;
use crate::core::shared::repositories::filter::Filter;
use crate::core::shared::repositories::filter::Filter::Expr;
use crate::core::shared::repositories::query::Query;

pub mod pagination;

impl From<Query> for Document {
    fn from(value: Query) -> Self {
        match value.filter {
            Expr(e) => {
                match e {
                    ExprStr(x) => doc! { x.field.as_str() : x.head.as_str() }
                }
            }
            Filter::None => doc! {}
        }
    }
}