mod test;
mod controllers;
mod common;

use common::error::Error;
use crate::common::server::make_server;


#[async_std::main]
async fn main() -> Result<(), Error> {
    let app = make_server().await?;
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
