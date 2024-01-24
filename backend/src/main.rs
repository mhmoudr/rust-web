use serde_json::json;
use sqlx::Pool;
use sqlx::PgPool;
use sqlx::query;
use tide::Request;
use tide::Server;
#[async_std::main]
async fn main() -> Result<(),Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL")?;

    let db_pool= Pool::connect(&db_url).await?;

    

    let mut app:Server<State> = Server::with_state(State{db_pool});
    app
        .at("/")
        .get(|req: Request<State>| async move {
            let conn = &req.state().db_pool;
            let row = query!("select 1 as one").fetch_one(conn).await?;
            dbg!(row);
            let res = json!(["Hey there"]);
            Ok(res)
        });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}


#[derive(Debug)]
#[derive(Clone)]
struct State{
    db_pool:PgPool,
}

#[derive(thiserror::Error,Debug)]
enum Error{
    #[error(transparent)]
    DBError(#[from] sqlx::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}