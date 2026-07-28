use sqlx::PgPool;

#[dervie(Clone)]
pub struct Appstate {
    pub db: PgPool,
    pub jwt_secret: String,
}
