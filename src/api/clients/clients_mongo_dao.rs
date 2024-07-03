use crate::api::shared::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};
use crate::api::clients::clients_dbo::{ClientDboEvent, ClientDboState};

pub type ClientsMongoDAO = EntityMongoDAO<ClientDboState>;
pub type ClientsEventMongoDAO = EventMongoDAO<ClientDboEvent>;