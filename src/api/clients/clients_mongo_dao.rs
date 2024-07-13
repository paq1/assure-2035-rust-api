use crate::api::clients::clients_dbo::{ClientDboEvent, ClientDboState};
use crate::api::shared::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};

pub type ClientsMongoDAO = EntityMongoDAO<ClientDboState>;
pub type ClientsEventMongoDAO = EventMongoDAO<ClientDboEvent>;