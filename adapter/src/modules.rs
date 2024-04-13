use kernel::{
    model::{r#match::Match, user::User},
    repository::{r#match::MatchRepository, user::UserRepository},
    traq::message::MessageTraqRepository,
};

use crate::{
    persistence::mariadb::Db, repository::DatabaseRepositoryImpl, traq::TraqRepositoryImpl,
};

pub struct RepositoriesModule {
    user_repository: DatabaseRepositoryImpl<User>,
    match_repository: DatabaseRepositoryImpl<Match>,
    message_traq_repository: TraqRepositoryImpl,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type MessageTraqRepo: MessageTraqRepository;
    type MatchRepo: MatchRepository;
    fn user_repository(&self) -> &Self::UserRepo;
    fn message_traq_repository(&self) -> &Self::MessageTraqRepo;
    fn match_repository(&self) -> &Self::MatchRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepositoryImpl<User>;
    type MatchRepo = DatabaseRepositoryImpl<Match>;
    type MessageTraqRepo = TraqRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
    fn message_traq_repository(&self) -> &Self::MessageTraqRepo {
        &self.message_traq_repository
    }
    fn match_repository(&self) -> &Self::MatchRepo {
        &self.match_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let access_token = std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN is not set");

        Self {
            user_repository: DatabaseRepositoryImpl::new(db.clone()),
            match_repository: DatabaseRepositoryImpl::new(db),
            message_traq_repository: TraqRepositoryImpl::new(access_token),
        }
    }
}
