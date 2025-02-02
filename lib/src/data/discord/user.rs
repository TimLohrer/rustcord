use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: i32,
    pub premium_type: i32,
    pub flags: i32,
    pub purchased_flags: Option<i32>,
    pub bot: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub global_name: Option<String>,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub mfa_enabled: bool,
    pub locale: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub verified: bool,
    pub bio: String,
    pub nsfw_allowed: Option<bool>,
    pub linked_users: Option<Vec<String>>,
    pub authenticator_types: Option<Vec<i32>>
}

impl User {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            username: String::new(),
            avatar: None,
            discriminator: String::new(),
            public_flags: 0,
            premium_type: 0,
            flags: 0,
            purchased_flags: None,
            bot: None,
            banner: None,
            accent_color: None,
            global_name: None,
            avatar_decoration_data: None,
            banner_color: None,
            mfa_enabled: false,
            locale: String::new(),
            email: None,
            phone: None,
            verified: false,
            bio: String::new(),
            nsfw_allowed: None,
            linked_users: None,
            authenticator_types: None
        }
    }
}