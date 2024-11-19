use std::{fmt, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DB {
    Postgres,
    QuestDB,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db: String,
    pub db_type: Option<DB>,
}

impl Into<String> for DB {
    fn into(self) -> String {
        match self {
            DB::Postgres => "pg".to_string(),
            DB::QuestDB => "quest".to_string(),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}@{}:{}/{}",
            self.host, self.port, self.user, self.password, self.db
        )
    }
}

impl Into<String> for Config {
    fn into(self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub postgres: Option<Config>,
    pub questdb: Option<Config>,
}

impl ConfigFile {
    pub fn new(config_path: &str) -> Result<ConfigFile, String> {
        let file = fs::read_to_string(config_path);
        if file.is_ok() {
            let mut parsed: ConfigFile = toml::from_str(&file.unwrap()).unwrap();
            parsed.postgres.as_mut().map(|c| c.db_type = Some(DB::Postgres));
            parsed.questdb.as_mut().map(|c| c.db_type = Some(DB::QuestDB));

            Ok(parsed)
        } else {
            Err(String::from("Could not read config file"))
        }
    }

    pub fn stub() -> ConfigFile {
        ConfigFile {
            postgres: Some(Config {
                host: "localhost".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "password".to_string(),
                db: "postgres".to_string(),
                db_type: Some(DB::Postgres),
            }),
            questdb: Some(Config {
                host: "localhost".to_string(),
                port: 8812,
                user: "admin".to_string(),
                password: "quest".to_string(),
                db: "qdb".to_string(),
                db_type: Some(DB::QuestDB),
            }),
        }
    }
}

impl Into<String> for ConfigFile {
    fn into(self) -> String {
        toml::to_string(&self).unwrap()
    }
}
