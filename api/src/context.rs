use sea_orm::DatabaseConnection;

#[derive(Clone, Default)]
pub struct Ctx {
    pub db: DatabaseConnection,
}

impl Ctx {
    pub async fn new() -> eyre::Result<Self> {
        let db = sea_orm::Database::connect(dotenvy::var("DATABASE_URL")?).await?;

        Ok(Self { db })
    }
}
