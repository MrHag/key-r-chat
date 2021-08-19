use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginReply {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserReply {
    pub id: u32,
    pub login: String,
    pub avatar_id: u32,
    pub about_me: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAvatarReply {
    pub id: u32,
    pub avatar: String,
}
