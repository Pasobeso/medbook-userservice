use anyhow::Result;
use diesel::{pg::PgConnection, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// ปรับพาธให้ตรงที่เก็บ migrations จริง ๆ
pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/infrastructure/postgres/migrations");

/// รัน migrations แบบ synchronous ใน thread แยก (เสถียร/ชัวร์)
pub async fn run_migrations_blocking(database_url: &str) -> Result<usize> {
    let url = database_url.to_string();

    let applied = tokio::task::spawn_blocking(move || -> Result<usize> {
        // 1) ต่อ DB แบบ sync
        let mut conn = PgConnection::establish(&url)
            .map_err(|e| anyhow::anyhow!("connect for migrations failed: {e}"))?;

        // 2) รันทุก migration ที่ยังไม่ถูก apply
        let versions = conn
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("running migrations failed: {e}"))?;

        Ok(versions.len())
    })
    .await
    .map_err(|e| anyhow::anyhow!("spawn_blocking join error: {e}"))??;

    Ok(applied)
}
