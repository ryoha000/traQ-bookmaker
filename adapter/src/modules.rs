use kernel::{model::user::User, repository::user::UserRepository};

use crate::{persistence::mariadb::Db, repository::DatabaseRepositoryImpl};

pub struct RepositoriesModule {
    user_repository: DatabaseRepositoryImpl<User>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    fn user_repository(&self) -> &Self::UserRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepositoryImpl<User>;
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        Self {
            user_repository: DatabaseRepositoryImpl::new(db),
        }
    }
}
