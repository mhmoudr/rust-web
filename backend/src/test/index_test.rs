use crate::common::server::make_server;
#[async_std::test]
async fn index_test() -> Result<(),Box<dyn std::error::Error> > {
    let app = make_server().await?;
    use tide_testing::TideTestingExt;
    let res = app.get("/").recv_string().await?;
    assert_eq!(res, "[\"Hey there\"]");
    assert_eq!(
        app.post("/missing").await?.status(),
        tide::http::StatusCode::NotFound
    );
    Ok(())
}

async fn not_found_test() -> Result<(),Box<dyn std::error::Error> > {
    let app = make_server().await?;
    use tide_testing::TideTestingExt;
    assert_eq!(
        app.post("/missing").await?.status(),
        tide::http::StatusCode::NotFound
    );
    Ok(())
}
