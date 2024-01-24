use tide::Request;
use crate::common::state::State;
use sqlx::query;
use serde_json::json;
pub async fn index(req: Request<State>) -> tide::Result {
    let conn = &req.state().db_pool;
    let row = query!("select 1 as one").fetch_one(conn).await?;
    dbg!(row);
    let res = json!(["Hey there"]);
    Ok(res.into())
}
