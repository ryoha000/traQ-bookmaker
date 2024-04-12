use std::sync::Arc;

use adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::mariadb::Db,
};
use app::usecase::user::UserUseCase;

pub struct Modules {
    user_use_case: UserUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let user_use_case = UserUseCase::new(repositories_module.clone());

        Self { user_use_case }
    }
}
