use sqlx::pool::Pool;
use sqlx::PgPool;
use tide::Server;
mod test;
mod controllers;
mod common;

use common::error::Error;
use common::state::State;
use controllers::index::index;

#[cfg(not(test))]
async fn make_db_pool()->Result<PgPool, Error>{
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(Pool::connect(&db_url).await?)
}

#[cfg(test)]
async fn make_db_pool() -> Result<PgPool, Error> {
    let db_url = std::env::var("DATABASE_URL_TEST")?;
    // let pool = Pool::connect(&db_url).await?;
    // Ok(pool)
    Ok(Pool::connect(&db_url).await?)
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    
    let db_pool = make_db_pool().await?;

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(index);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
