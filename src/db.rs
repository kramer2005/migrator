use std::fs;

use sqlx::{postgres::PgPoolOptions, Acquire, Pool, Postgres};

use crate::{
    parser::Cli,
    utils::{config::Config, operations::add},
};

pub async fn transaction(pool: Pool<Postgres>, query: &str) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await.unwrap();
    let mut tx = conn.begin().await.unwrap();

    match sqlx::query(query).execute(&mut *tx).await {
        Ok(_) => {
            tx.commit().await.unwrap();
            conn.close().await.unwrap();
            return Ok(());
        }
        Err(e) => {
            tx.rollback().await.unwrap();
            conn.close().await.unwrap();
            return Err(e);
        }
    }
}

pub async fn check(
    pool: Pool<Postgres>,
    cli: &Cli,
    config: &Config,
    print: bool,
) -> Option<String> {
    let db: String = config.db_type.clone().unwrap().into();
    let path = format!("{}/{}", cli.base_path(), db);
    let plan = read_plan(cli, config);

    for query in plan {
        let sql = fs::read_to_string(format!("{}/verify/{}.sql", path, query)).unwrap();

        let result = sqlx::query(&sql).execute(&pool).await;

        if result.is_err() {
            return Some(query);
        }

        if print {
            println!("{}: OK", query);
        }
    }

    None
}

pub async fn deploy(pool: Pool<Postgres>, cli: &Cli, config: &Config, checkpoint: Option<String>) {
    let db: String = config.db_type.clone().unwrap().into();
    let path = format!("{}/{}", cli.base_path(), db);
    let mut plan = read_plan(cli, config);

    if checkpoint.is_some() {
        let checkpoint = checkpoint.clone().unwrap();
        let index = plan.iter().position(|x| x == &checkpoint).unwrap();
        plan = plan.split_off(index);
    }

    for query in plan {
        let deploy = fs::read_to_string(format!("{}/deploy/{}.sql", path, query)).unwrap();

        transaction(pool.clone(), &deploy).await.unwrap();

        println!("{}: OK", query);
    }
}

pub async fn revert(pool: Pool<Postgres>, cli: &Cli, config: &Config, checkpoint: Option<String>) {
    let db: String = config.db_type.clone().unwrap().into();
    let path = format!("{}/{}", cli.base_path(), db);
    let mut plan = read_plan(cli, config);
    plan.reverse();

    if checkpoint.is_some() {
        let checkpoint = checkpoint.clone().unwrap();
        let index = plan.iter().position(|x| x == &checkpoint).unwrap();
        plan = plan.split_off(index + 1);
    }

    for query in plan {
        let revert = fs::read_to_string(format!("{}/revert/{}.sql", path, query)).unwrap();

        transaction(pool.clone(), &revert).await.unwrap();

        println!("{}: OK", query);
    }
}

pub async fn connect(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    let conn_str: String = config.clone().into();

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&conn_str)
        .await;

    pool
}

pub fn read_plan(cli: &Cli, config: &Config) -> Vec<String> {
    let db: String = config.db_type.clone().unwrap().into();
    let path = format!("{}/{}/migrator.plan", cli.base_path(), db);
    let file = fs::read_to_string(path);

    if file.is_ok() {
        file.unwrap().lines().map(|l| l.to_string()).collect()
    } else {
        vec![]
    }
}

pub async fn operate(config: Config) {
    let cli = Cli::new();

    if cli.args[1] == "add" {
        println!("Adding migration");

        return add(&cli, &config);
    }

    if cli.args[1] == "check" {
        let result = check(connect(&config).await.unwrap(), &cli, &config, true).await;

        if result.is_some() {
            println!("Migration failed: {}", result.unwrap());
        } else {
            println!("Nothing to deploy");
        }

        return;
    }

    if cli.args[1] == "deploy" {
        let checkpoint = check(connect(&config).await.unwrap(), &cli, &config, false).await;

        if checkpoint.is_none() {
            println!("Nothing to deploy");
            return;
        }

        deploy(connect(&config).await.unwrap(), &cli, &config, checkpoint).await;

        return;
    }

    if cli.args[1] == "revert" {
        let checkpoint = check(connect(&config).await.unwrap(), &cli, &config, false).await;

        revert(connect(&config).await.unwrap(), &cli, &config, checkpoint).await;

        return;
    }
}
