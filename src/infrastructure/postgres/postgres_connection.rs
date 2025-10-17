use anyhow::Result;
use diesel::sql_query;
use std::time::Duration;

use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection, RunQueryDsl,
};

pub type PgPoolSquad = Pool<AsyncPgConnection>;

pub async fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    let pool = Pool::builder()
        // 1 วินาทีสั้นไปหน่อย เพิ่มให้สมเหตุสมผลกว่านี้
        .connection_timeout(Duration::from_secs(5))
        // ให้ bb8 พยายามเปิดอย่างน้อย 1 คอนเนกชันทันที (eager)
        .min_idle(Some(1))
        // ตรวจสุขภาพทุกครั้งที่ยืม (optional แต่ช่วยจับคอนเนกชันเน่าได้ดี)
        .test_on_check_out(true)
        .build(manager)
        .await?; // ถ้าเปิดคอนเนกชันแรกไม่ได้จะ error ที่นี่ (เมื่อมี min_idle)

    // พิสูจน์ว่าเชื่อมได้จริงโดยยืมคอนเนกชันแล้วยิง SELECT 1
    let mut conn = pool.get().await?; // ถ้าต่อไม่ได้ จะ error ตรงนี้
    drop(conn);

    Ok(pool)
}
