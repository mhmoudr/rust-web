#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let db_url = std::env::var("DATABASE_URL");
    dbg!(db_url);

    
}
