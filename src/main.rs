use parser::Cli;
use utils::{config::ConfigFile, help::help};

mod parser;
mod utils;
mod db;



#[tokio::main]
async fn main() {
    let cli = Cli::new();
    if cli.params.len() < 2 || cli.get_long_flag("--help").is_some() {
        help();
        return;
    }

    if cli.get_long_flag("--init").is_some() {
        utils::init(&cli.base_path());
        return;
    }

    let config = ConfigFile::new(
        format!("{}/config.toml", cli.base_path()).as_str(),
    );

    if config.is_err() {
        println!("{}", config.err().unwrap());
        return;
    }

    let config = config.unwrap();

    if cli.args[0] == "pg" {
        db::operate(config.postgres.unwrap()).await;
    } else if cli.args[0] == "quest" {
        db::operate(config.questdb.unwrap()).await;
    } else {
        help();
    }

}
