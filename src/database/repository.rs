use diesel_async::{
    AsyncConnection,
    AsyncSqliteConnection,
};

pub struct Database {
    connection: AsyncSqliteConnection,
}

impl Database {
    pub async fn new(database_url: &str) -> diesel::QueryResult<Self> {
        let connection = AsyncSqliteConnection::establish(database_url).await?;

        Ok(Self { connection })
    }
}