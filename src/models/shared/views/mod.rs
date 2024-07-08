pub mod command_handler_view;
pub mod states_views;
pub mod entities;
pub mod get_view;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct DataWrapperView<T>
where
    T: Serialize + Clone,
{
    pub data: T,
}

