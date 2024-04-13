use std::sync::Arc;

use adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::mariadb::Db,
};
use app::usecase::{message::help::HelpUseCase, user::UserUseCase};

pub struct Modules {
    bot_user_id: String,
    user_use_case: UserUseCase<RepositoriesModule>,
    help_use_case: HelpUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    fn bot_user_id(&self) -> &str;

    type RepositoriesModule: RepositoriesModuleExt;

    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule>;
    fn help_use_case(&self) -> &HelpUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    fn bot_user_id(&self) -> &str {
        &self.bot_user_id
    }

    type RepositoriesModule = RepositoriesModule;

    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_use_case
    }
    fn help_use_case(&self) -> &HelpUseCase<Self::RepositoriesModule> {
        &self.help_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let bot_user_id = std::env::var("BOT_USER_ID").expect("BOT_USER_ID is not set");

        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let user_use_case = UserUseCase::new(repositories_module.clone());

        let help_use_case = HelpUseCase::new(repositories_module.clone());

        Self {
            bot_user_id,
            user_use_case,
            help_use_case,
        }
    }
}
