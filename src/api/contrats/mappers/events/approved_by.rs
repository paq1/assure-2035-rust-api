use crate::api::contrats::contrats_dbo::ApprovedByDbo;
use crate::core::contrats::data::ApprovedBy;

impl From<ApprovedByDbo> for ApprovedBy {
    fn from(value: ApprovedByDbo) -> Self {
        Self {
            uid: value.uid,
            display_name: value.display_name,
            email: value.email,
        }
    }
}

impl From<ApprovedBy> for ApprovedByDbo {
    fn from(value: ApprovedBy) -> Self {
        Self {
            uid: value.uid,
            display_name: value.display_name,
            email: value.email,
        }
    }
}
