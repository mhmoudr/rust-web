use serde::Serialize;
use tide::Request;
use uuid::Uuid;
use crate::common::state::State;
use sqlx::query_as;
use serde_json::json;
pub async fn users(req: Request<State>) -> tide::Result {
    let conn = &req.state().db_pool;
    let users = query_as!(User, "select id, name, email from users").fetch_all(conn).await?;
    let res = json!(&users);
    Ok(res.into())
}

#[derive(Debug, Serialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}