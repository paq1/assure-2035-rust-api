use crate::api::contrats::contrats_dbo::{ContratDboEvent, ContratDboState};
use crate::api::shared::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};

pub type ContratsMongoDAO = EntityMongoDAO<ContratDboState>;
pub type ContratsEventMongoDAO = EventMongoDAO<ContratDboEvent>;