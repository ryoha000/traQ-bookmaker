use derive_new::new;
use kernel::model::{user::NewUser, Id};

#[derive(new)]
pub struct CreateUser {
    pub traq_id: String,
    pub traq_display_id: String,
    pub channel_id: String,
}

const INITIAL_BALANCE: i32 = 10_000;

impl From<CreateUser> for NewUser {
    fn from(c: CreateUser) -> Self {
        NewUser::new(
            Id::gen(),
            c.traq_id,
            c.traq_display_id,
            c.channel_id,
            INITIAL_BALANCE,
        )
    }
}
