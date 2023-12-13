use crate::{ReservationManager, Rsvp};
use abi::Validate;
use async_trait::async_trait;
use sqlx::{PgPool, Row};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::Reservation;
    use sqlx::PgPool;
    use sqlx_db_tester::TestDb;

    #[tokio::test]
    async fn reserve_should_work_for_valid_window() {
        let tdb = get_tdb();
        let pool = tdb.get_pool().await;
        let (rsvp, _manager) = make_tyr_reservation(pool).await;
        assert!(rsvp.id != 0);
    }

    fn get_tdb() -> TestDb {
        TestDb::new("10.11.32.24", 5432, "tester", "test123", "../migrations")
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
