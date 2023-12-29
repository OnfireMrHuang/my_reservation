use crate::{ReservationManager, Rsvp};
use abi::Validate;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tokio::sync::mpsc;

// 添加reservationManager方法
impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Rsvp for ReservationManager {
    // 实现预留预定资源接口
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        // 第一步，校验预定
        rsvp.validate()?;

        let status = abi::ReservationStatus::try_from(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let timespan = rsvp.get_timespan();

        // 生成insert into语句并将预定信息插入到数据库中
        let row = sqlx::query(
            "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?;

        rsvp.id = row.get(0);

        Ok(rsvp)
    }

    // 实现修改状态接口
    async fn change_status(&self, id: abi::ReservationId) -> Result<abi::Reservation, abi::Error> {
        // 如果当前状态是pending状态，则更改为确认状态，否则什么也不做
        id.validate()?;
        let rsvp: abi::Reservation = sqlx::query_as("UPDATE rsvp.reservations SET status = 'confirmed' WHERE id = $1 AND status = 'pending' RETURNING *")
            .bind(id)
            .fetch_one(&self.pool).await?;
        Ok(rsvp)
    }

    // 实现更新备注接口
    async fn update_note(
        &self,
        id: abi::ReservationId,
        note: String,
    ) -> Result<abi::Reservation, abi::Error> {
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("UPDATE rsvp.reservations SET note = $2 WHERE id = $1 RETURNING *")
                .bind(id)
                .bind(note)
                .fetch_one(&self.pool)
                .await?;
        Ok(rsvp)
    }

    // 实现get接口
    async fn get(&self, id: abi::ReservationId) -> Result<abi::Reservation, abi::Error> {
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("SELECT * FROM rsvp.reservations WHERE id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;
        Ok(rsvp)
    }

    // 实现删除接口
    async fn delete(&self, id: abi::ReservationId) -> Result<abi::Reservation, abi::Error> {
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("DELETE FROM rsvp.reservations WHERE id = $1 RETURNING *")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;
        Ok(rsvp)
    }

    // 实现查询接口
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, abi::Error>> {
        let pool = self.pool.clone();
        // 声明一个channel
        let (tx, rx) = mpsc::channel(128);

        // 开启一个异步任务查询预订记录
        tokio::spawn(async move {
            let sql = query
        })
    }

    // 实现过滤接口
    async fn filter(
        &self,
        user_id: Option<String>,
        resource_id: Option<String>,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        status: Option<abi::ReservationStatus>,
    ) -> Result<Vec<abi::Reservation>, abi::Error> {
        let mut query = "SELECT * FROM rsvp.reservations WHERE 1 = 1".to_string();
        let mut params = Vec::new();

        if let Some(user_id) = user_id {
            query += " AND user_id = $1";
            params.push(user_id);
        }

        if let Some(resource_id) = resource_id {
            query += " AND resource_id = $2";
            params.push(resource_id);
        }

        if let Some(start) = start {
            query += " AND timespan && tstzrange($3, $4)";
            params.push(start);
            params.push(end.unwrap());
        }

        if let Some(status) = status {
            query += " AND status = $5::rsvp.reservation_status";
            params.push(status.to_string());
        }

        let rsvps: Vec<abi::Reservation> = sqlx::query_as(&query)
            .bind(params)
            .fetch_all(&self.pool)
            .await?;

        Ok(rsvps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestPg;
    use abi::Reservation;
    use sqlx::{Connection, PgConnection, PgPool};
    use std::path::Path;

    fn get_tdb() -> TestPg {
        TestPg::new(
            "postgres://postgres:Huang2023@10.11.32.24:5432".to_string(),
            Path::new("../migrations"),
        )
    }

    #[tokio::test]
    async fn reserve_should_work_for_valid_window() {
        let tdb = get_tdb();
        let pool = tdb.get_pool().await;
        let (rsvp, _manager) = make_tyr_reservation(pool).await;
        assert!(rsvp.id != 0);
    }

    async fn make_tyr_reservation(pool: PgPool) -> (Reservation, ReservationManager) {
        make_reservation(
            pool,
            "tyrid",
            "ocean-view-room-713",
            "2022-12-25T15:00:00-0700",
            "2022-12-28T12:00:00-0700",
            "I'll arrive at 3pm. Please help to upgrade to execuitive room if possible.",
        )
        .await
    }

    async fn make_alice_reservation(pool: PgPool) -> (Reservation, ReservationManager) {
        make_reservation(
            pool,
            "aliceid",
            "ixia-test-1",
            "2023-01-25T15:00:00-0700",
            "2023-02-25T12:00:00-0700",
            "I need to book this for xyz project for a month.",
        )
        .await
    }

    async fn make_reservation(
        pool: PgPool,
        uid: &str,
        rid: &str,
        start: &str,
        end: &str,
        note: &str,
    ) -> (Reservation, ReservationManager) {
        let manager = ReservationManager::new(pool.clone());
        let rsvp = abi::Reservation::new_pending(
            uid,
            rid,
            start.parse().unwrap(),
            end.parse().unwrap(),
            note,
        );
        (manager.reserve(rsvp).await.unwrap(), manager)
    }
}
