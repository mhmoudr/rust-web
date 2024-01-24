use crate::common::state::State;
use sqlx::pool::Pool;
use tide::Server;
use crate::controllers::index::index;

#[async_std::test]
async fn test_server() -> Result<(),Box<dyn std::error::Error> > {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = Pool::connect(&db_url).await?;

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(index);

    use tide_testing::TideTestingExt;
    let res = app.get("/").recv_string().await?;
    assert_eq!(res, "[\"Hey there\"]");
    assert_eq!(
        app.post("/missing").await?.status(),
        tide::http::StatusCode::NotFound
    );
    Ok(())
}
