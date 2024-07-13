pub mod command_handler_view;
pub mod states_views;
pub mod entities;
pub mod get_view;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct DataWrapperView<T>
where
    T: Serialize + Clone,
{
    pub data: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkView {
    #[serde(flatten)]
    pub links: HashMap<String, String>,
}

