use derive_new::new;
use std::marker::PhantomData;
use uuid::Uuid;

pub mod message;
pub mod user;

#[derive(new, Debug, Clone)]
pub struct Id<T> {
    pub value: String,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn gen() -> Id<T> {
        Id::new(Uuid::new_v4().to_string())
    }
}
