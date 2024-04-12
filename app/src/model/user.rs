use derive_new::new;
use kernel::model::{user::NewUser, Id};

#[derive(new)]
pub struct CreateUser {
    pub id: String,
    pub traq_id: String,
}

const INITIAL_BALANCE: i32 = 10_000;

impl From<CreateUser> for NewUser {
    fn from(c: CreateUser) -> Self {
        NewUser::new(Id::new(c.id), c.traq_id, INITIAL_BALANCE)
    }
}
