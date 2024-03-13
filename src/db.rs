use crate::SESSION_SECRET_KEY;
use rand::Rng;
use sqlx::postgres::PgPool;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Generates and inserts a session secret key into a PostgreSQL database table if it does not already exist.
///
/// # Arguments
///
/// * `pool` - A reference to a `PgPool` object representing the connection pool to the PostgreSQL database.
///
/// # Returns
///
/// * `Ok(())` - If the session secret key already exists in the database or if the function successfully generates and inserts the session secret key.
/// * `Err(sqlx::Error)` - If there is an error executing the SQL queries or interacting with the database.
pub async fn initialize_configs(pool: &PgPool) -> Result<(), sqlx::Error> {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM settings WHERE key = $1)")
        .bind(SESSION_SECRET_KEY)
        .fetch_one(pool)
        .await?;

    if exists.0 {
        info!("Session secret already set, skipping generation...");
        return Ok(());
    }

    warn!("No session secret set, generating one now...");

    let secure_random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    sqlx::query("INSERT INTO settings (id, key, value) VALUES ($1, $2, $3)")
        .bind(Uuid::new_v4())
        .bind(SESSION_SECRET_KEY)
        .bind(secure_random_string)
        .execute(pool)
        .await?;

    info!("Session secret generated successfully!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Setting;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Error;
    use std::env;
    #[tokio::test]
    async fn test_generate_session_secret() -> Result<(), Error> {
        dotenv::dotenv().ok();
        tracing_subscriber::fmt::init();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Create a mock pool
        let pool = PgPoolOptions::new().connect(&database_url).await?;

        // Call the function under test
        initialize_configs(&pool).await?;

        // Assert that the session secret is generated and inserted into the database
        let row: Setting = sqlx::query_as("SELECT * FROM settings WHERE key = $1")
            .bind(&crate::SESSION_SECRET_KEY)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.key, crate::SESSION_SECRET_KEY);

        Ok(())
    }
}
