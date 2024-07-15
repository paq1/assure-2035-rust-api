use crate::api::contrats::contrats_dbo::UserInfoDbo;
use crate::core::contrats::data::UserInfo;

impl From<UserInfoDbo> for UserInfo {
    fn from(value: UserInfoDbo) -> Self {
        Self {
            uid: value.uid,
            display_name: value.display_name,
            email: value.email,
        }
    }
}

impl From<UserInfo> for UserInfoDbo {
    fn from(value: UserInfo) -> Self {
        Self {
            uid: value.uid,
            display_name: value.display_name,
            email: value.email,
        }
    }
}
