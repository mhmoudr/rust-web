use crate::common::server::make_server;
use serde_json::json;
use tide_testing::TideTestingExt;

#[async_std::test]
async fn users_test() -> Result<(),Box<dyn std::error::Error> > {
    let app = make_server().await?;

    let res = app.get("/users").recv_string().await?;
   
    assert_eq!(res, json!([]).to_string());
    assert_eq!(
        app.post("/missing").await?.status(),
        tide::http::StatusCode::NotFound
    );
    Ok(())
}
