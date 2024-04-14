use kernel::{
    model::{candidate::Candidate, r#match::Match, user::User},
    repository::{candidate::CandidateRepository, r#match::MatchRepository, user::UserRepository},
    traq::message::MessageTraqRepository,
};

use crate::{
    persistence::mariadb::Db, repository::DatabaseRepositoryImpl, traq::TraqRepositoryImpl,
};

pub struct RepositoriesModule {
    user_repository: DatabaseRepositoryImpl<User>,
    match_repository: DatabaseRepositoryImpl<Match>,
    candidate_repository: DatabaseRepositoryImpl<Candidate>,
    message_traq_repository: TraqRepositoryImpl,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type MatchRepo: MatchRepository;
    type CandidateRepo: CandidateRepository;
    type MessageTraqRepo: MessageTraqRepository;
    fn user_repository(&self) -> &Self::UserRepo;
    fn match_repository(&self) -> &Self::MatchRepo;
    fn candidate_repository(&self) -> &Self::CandidateRepo;
    fn message_traq_repository(&self) -> &Self::MessageTraqRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepositoryImpl<User>;
    type MatchRepo = DatabaseRepositoryImpl<Match>;
    type CandidateRepo = DatabaseRepositoryImpl<Candidate>;
    type MessageTraqRepo = TraqRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
    fn match_repository(&self) -> &Self::MatchRepo {
        &self.match_repository
    }
    fn candidate_repository(&self) -> &Self::CandidateRepo {
        &self.candidate_repository
    }
    fn message_traq_repository(&self) -> &Self::MessageTraqRepo {
        &self.message_traq_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let access_token = std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN is not set");

        Self {
            user_repository: DatabaseRepositoryImpl::new(db.clone()),
            match_repository: DatabaseRepositoryImpl::new(db.clone()),
            candidate_repository: DatabaseRepositoryImpl::new(db.clone()),
            message_traq_repository: TraqRepositoryImpl::new(access_token),
        }
    }
}
