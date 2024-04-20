use derive_new::new;

use super::{channel::Channel, Id};

#[derive(new, Debug)]
pub struct User {
    pub id: Id<User>,
    pub traq_id: String,
    pub traq_display_id: String,
    pub channel_id: String,
    pub balance: i32,
}

#[derive(new, Debug)]
pub struct NewUser {
    pub id: Id<User>,
    pub traq_id: String,
    pub traq_display_id: String,
    pub channel_id: String,
    pub balance: i32,
}

#[derive(new, Debug)]
pub struct FindUser {
    pub traq_id: String,
    pub channel_id: Id<Channel>,
}
