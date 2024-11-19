use std::fs;

use crate::{db::read_plan, parser::Cli};

use super::config::Config;

pub fn add(cli: &Cli, config: &Config) {
    let db: String = config.db_type.clone().unwrap().into();
    let path = format!("{}/{}", cli.base_path(), db);

    let name = cli.args[2].clone();
    let plan = read_plan(cli, config);

    let order = plan.len() + 1;

    let deploy = format!("{}/deploy/{}-{}.sql", path, order, name);
    fs::write(deploy, "-- Write your migration here").unwrap();

    let revert = format!("{}/revert/{}-{}.sql", path, order, name);
    fs::write(revert, "-- Write your rollback here").unwrap();

    let verify = format!("{}/verify/{}-{}.sql", path, order, name);
    fs::write(verify, "-- Write your verification here").unwrap();

    let mut plan = plan;
    plan.push(format!("{}-{}", order, name));

    let plan = plan.join("\n");
    let path = format!("{}/{}/migrator.plan", cli.base_path(), db);
    fs::write(path, plan).unwrap();
    
}