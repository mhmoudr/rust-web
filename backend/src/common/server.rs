use crate::controllers::index::index;
use crate::controllers::users::users;

use super::error::Error;
use super::state::State;
use sqlx::PgPool;
use sqlx::Pool;
use tide::Server;

#[cfg(not(test))]
async fn make_db_pool()->Result<PgPool, Error>{
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(Pool::connect(&db_url).await?)
}

#[cfg(test)]
async fn make_db_pool() -> Result<PgPool, Error> {
    let db_url = std::env::var("DATABASE_URL_TEST")?;
    Ok(Pool::connect(&db_url).await?)
}

pub async fn make_server() -> Result<Server<State>,Error>{
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_pool = make_db_pool().await?;
    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(index);
    app.at("/users").get(users);

    Ok(app)
}