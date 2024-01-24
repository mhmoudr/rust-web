use sqlx::PgPool;
#[derive(Debug, Clone)]
pub struct State {
    pub db_pool: PgPool,
}