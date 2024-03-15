use crate::models::PayloadFireResult;

pub async fn get(
    pool: &sqlx::PgPool,
    probe_id: &str,
) -> Result<Vec<PayloadFireResult>, sqlx::Error> {
    let result = sqlx::query_as::<_, PayloadFireResult>(
        "SELECT * FROM payload_fire_results WHERE probe_id = $1",
    )
    .bind(probe_id)
    .fetch_all(pool)
    .await;
    result
}
