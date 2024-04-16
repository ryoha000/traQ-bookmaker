use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::mariadb::Db;

pub mod bet;
pub mod candidate;
pub mod r#match;
pub mod user;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    db: Db,
    _marker: PhantomData<T>,
}
