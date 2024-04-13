use std::env;
use std::time::Duration;

use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone)]
pub struct Db(pub(crate) DatabaseConnection);

fn must_get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} is not set!", key))
}

impl Db {
    pub async fn new() -> Db {
        let username = must_get_env("NS_MARIADB_USERNAME");
        let password = must_get_env("NS_MARIADB_PASSWORD");
        let hostname = must_get_env("NS_MARIADB_HOSTNAME");
        let database = must_get_env("NS_MARIADB_DATABASE");

        let mut opt = ConnectOptions::new(format!(
            "mysql://{}:{}@{}/{}",
            username, password, hostname, database
        ));
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        let db = Database::connect(opt).await.unwrap_or_else(|e| {
            panic!("Failed to connect to MariaDB: {}", e);
        });

        migration::Migrator::up(&db, None)
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to migrate: {}", e);
            });
        Db(db)
    }
}
