use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataWrapperView<T>
where
    T: Serialize + Clone,
{
    pub data: T
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct EntityView<T>
where
    T: Serialize + Clone
{
    pub r#type: String,
    pub id: String,
    pub attributes: T,
}