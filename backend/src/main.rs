use sqlx::pool::Pool;
use tide::Server;
mod test;
mod controllers;
mod common;

use common::error::Error;
use common::state::State;
use controllers::index::index;
#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL")?;

    let db_pool = Pool::connect(&db_url).await?;

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(index);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
