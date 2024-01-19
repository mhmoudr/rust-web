#[async_std::main]
async fn main() -> Result<(),Error> {
    dotenv::dotenv().ok();

    let mut app = tide::new();
    app.at("/").get(|_| async move {Ok("Hey there!")});

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[derive(thiserror::Error,Debug)]
enum Error{
    #[error(transparent)]
    DBError(#[from] sqlx::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}