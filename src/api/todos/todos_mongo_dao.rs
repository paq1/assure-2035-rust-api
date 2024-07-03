use crate::api::shared::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};
use crate::api::todos::todo_dbo::{TodoDboEvent, TodoDboState};

pub type TodosMongoDAO = EntityMongoDAO<TodoDboState>;
pub type TodosEventMongoDAO = EventMongoDAO<TodoDboEvent>;