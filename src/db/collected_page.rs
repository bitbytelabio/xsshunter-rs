use crate::models::CollectedPage;

pub async fn get(pool: &sqlx::PgPool, uri: &str) -> Result<Vec<CollectedPage>, sqlx::Error> {
    let result = sqlx::query_as::<_, CollectedPage>("SELECT * FROM collected_pages WHERE uri = $1")
        .bind(uri)
        .fetch_all(pool)
        .await;
    result
}

pub async fn create(
    pool: &sqlx::PgPool,
    uri: &str,
    html: &str,
) -> Result<CollectedPage, sqlx::Error> {
    let result = sqlx::query_as(
        "INSERT INTO collected_pages (uri, html) VALUES ($1, $2) RETURNING id, uri, html",
    )
    .bind(uri)
    .bind(html)
    .fetch_one(pool)
    .await;
    result
}

pub async fn delete(pool: &sqlx::PgPool, uri: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM collected_pages WHERE uri = $1")
        .bind(uri)
        .execute(pool)
        .await?;
    Ok(())
}
