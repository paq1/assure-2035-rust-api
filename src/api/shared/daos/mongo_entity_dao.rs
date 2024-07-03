use crate::api::shared::daos::dbos::{EntityDBO, EventDBO};
use crate::api::shared::daos::mongo_dao::MongoDAO;

pub type EntityMongoDAO<DATADBO> = MongoDAO<EntityDBO<DATADBO, String>>;
pub type EventMongoDAO<EVENTDBO> = MongoDAO<EventDBO<EVENTDBO, String>>;
