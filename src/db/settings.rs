use crate::models::Setting;

pub async fn get(pool: &sqlx::PgPool, key: &str) -> Result<Setting, sqlx::Error> {
    let setting: Setting = sqlx::query_as("SELECT * FROM settings WHERE key = $1")
        .bind(key)
        .fetch_one(pool)
        .await?;

    Ok(setting)
}

pub async fn create(pool: &sqlx::PgPool, key: &str, value: &str) -> Result<Setting, sqlx::Error> {
    let setting: Setting = sqlx::query_as(
        "INSERT INTO settings (key, value) VALUES ($1, $2) RETURNING id, key, value",
    )
    .bind(key)
    .bind(value)
    .fetch_one(pool)
    .await?;

    Ok(setting)
}

pub async fn update(pool: &sqlx::PgPool, key: &str, value: &str) -> Result<Setting, sqlx::Error> {
    let setting: Setting =
        sqlx::query_as("UPDATE settings SET value = $1 WHERE key = $2 RETURNING id, key, value")
            .bind(value)
            .bind(key)
            .fetch_one(pool)
            .await?;

    Ok(setting)
}

pub async fn delete(pool: &sqlx::PgPool, key: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM settings WHERE key = $1")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}

// Returns a Setting object when given a valid key and database connection
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::types::chrono::{DateTime, Utc};
    use sqlx::types::Uuid;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_get_setting() {
        dotenv::dotenv().ok();
        // Arrange
        let pool = PgPoolOptions::new()
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let result = get(&pool, SESSION_SECRET_KEY).await;

        assert!(result.is_ok());
        let setting = result.unwrap();
        assert_eq!(setting.key, SESSION_SECRET_KEY);
    }

    #[test(tokio::test)]
    async fn test_create_setting() {
        dotenv::dotenv().ok();
        // Arrange
        let pool = PgPoolOptions::new()
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let result = create(&pool, "TEST_KEY", "TEST_VALUE").await;

        assert!(result.is_ok());
        let setting = result.unwrap();
        assert_eq!(setting.key, "TEST_KEY");
        assert_eq!(setting.value, "TEST_VALUE");
    }

    #[test(tokio::test)]
    async fn test_update_setting() {
        dotenv::dotenv().ok();
        let pool = PgPoolOptions::new()
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let result = create(&pool, "TEST_KEY", "TEST_VALUE1").await;

        let result = update(&pool, "TEST_KEY", "TEST_VALUE2").await;

        assert!(result.is_ok());
        let setting = result.unwrap();
        assert_eq!(setting.key, "TEST_KEY");
        assert_eq!(setting.value, "TEST_VALUE2");
    }

    #[test(tokio::test)]
    async fn test_delete_setting() {
        dotenv::dotenv().ok();
        let pool = PgPoolOptions::new()
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let result = create(&pool, "TEST_KEY", "TEST_VALUE").await;

        let result = delete(&pool, "TEST_KEY").await;

        assert!(result.is_ok());
    }
}
