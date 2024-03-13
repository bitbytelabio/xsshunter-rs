use std::process::exit;

use crate::utils::{get_hashed_password, get_secure_random_string};
use crate::{ADMIN_PASSWORD_SETTINGS_KEY, CORRELATION_API_SECRET_SETTINGS_KEY, SESSION_SECRET_KEY};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::postgres::PgPool;
use tracing::{error, info, warn};
use uuid::Uuid;

pub mod collected_page;
pub mod settings;

async fn initialize_configs(pool: &PgPool) -> Result<(), sqlx::Error> {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM settings WHERE key = $1)")
        .bind(SESSION_SECRET_KEY)
        .fetch_one(pool)
        .await?;

    if exists.0 {
        info!("Session secret already set, skipping generation...");
        return Ok(());
    }

    info!("No session secret set, generating one now...");

    let secure_random_string = get_secure_random_string(32);

    sqlx::query("INSERT INTO settings (id, key, value) VALUES ($1, $2, $3)")
        .bind(Uuid::new_v4())
        .bind(SESSION_SECRET_KEY)
        .bind(secure_random_string)
        .execute(pool)
        .await?;

    info!("Session secret generated successfully!");

    Ok(())
}

async fn initialize_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM settings WHERE key = $1)")
        .bind(ADMIN_PASSWORD_SETTINGS_KEY)
        .fetch_one(pool)
        .await?;

    if exists.0 {
        info!("Admin user already set, skipping generation...");
        return Ok(());
    }

    info!("No admin user set, generating one now...");
    let password = get_secure_random_string(32);

    warn!("Admin user generated with password: {}", password);

    let bcrypt_hash = get_hashed_password(&password).expect("Failed to hash password");

    sqlx::query("INSERT INTO settings (id, key, value) VALUES ($1, $2, $3)")
        .bind(Uuid::new_v4())
        .bind(ADMIN_PASSWORD_SETTINGS_KEY)
        .bind(bcrypt_hash)
        .execute(pool)
        .await?;

    info!("Admin user generated successfully!");

    Ok(())
}

async fn initialize_correlation_api(pool: &PgPool) -> Result<(), sqlx::Error> {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM settings WHERE key = $1)")
        .bind(CORRELATION_API_SECRET_SETTINGS_KEY)
        .fetch_one(pool)
        .await?;

    if exists.0 {
        info!("Correlation API secret already set, skipping generation...");
        return Ok(());
    }

    info!("No correlation API secret set, generating one now...");

    let api_key = get_secure_random_string(64);

    sqlx::query("INSERT INTO settings (id, key, value) VALUES ($1, $2, $3)")
        .bind(Uuid::new_v4())
        .bind(CORRELATION_API_SECRET_SETTINGS_KEY)
        .bind(api_key)
        .execute(pool)
        .await?;

    info!("Correlation API secret generated successfully!");

    Ok(())
}

pub async fn initialize_database(pool: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");

    match initialize_configs(&pool).await {
        Ok(_) => info!("Session secret generated successfully!"),
        Err(e) => {
            error!("Error generating session secret: {:?}", e);
            exit(1)
        }
    }

    match initialize_correlation_api(&pool).await {
        Ok(_) => info!("Correlation API secret generated successfully!"),
        Err(e) => {
            error!("Error generating correlation API secret: {:?}", e);
            exit(1)
        }
    }

    match initialize_users(&pool).await {
        Ok(_) => info!("Admin user generated successfully!"),
        Err(e) => {
            error!("Error generating admin user: {:?}", e);
            exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;
    use crate::models::Setting;

    use sqlx::postgres::PgPoolOptions;
    use sqlx::Error;
    use std::env;
    #[test(tokio::test)]
    async fn test_generate_session_secret() -> Result<(), Error> {
        dotenv::dotenv().ok();

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

    #[test(tokio::test)]
    async fn generates_admin_user_with_valid_password() -> Result<(), Error> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new().connect(&database_url).await?;
        let _hash = hash("password123", DEFAULT_COST).unwrap();

        initialize_users(&pool).await?;

        let row: Setting = sqlx::query_as("SELECT * FROM settings WHERE key = $1")
            .bind(&crate::ADMIN_PASSWORD_SETTINGS_KEY)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.key, crate::ADMIN_PASSWORD_SETTINGS_KEY);
        // assert_eq!(row.value, hash);

        Ok(())
    }

    #[test(tokio::test)]
    async fn generates_correlation_api_secret() -> Result<(), Error> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new().connect(&database_url).await?;

        initialize_correlation_api(&pool).await?;

        let row: Setting = sqlx::query_as("SELECT * FROM settings WHERE key = $1")
            .bind(&crate::CORRELATION_API_SECRET_SETTINGS_KEY)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.key, crate::CORRELATION_API_SECRET_SETTINGS_KEY);

        Ok(())
    }
}
