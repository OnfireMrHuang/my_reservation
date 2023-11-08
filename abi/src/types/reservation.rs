use super::validate_range;
use crate::{convert_to_timestamp, get_timespan, Error, Reservation, ReservationStatus, Validate};
use chrono::{DateTime, FixedOffset, Utc};
use sqlx::postgres::types::PgRange;

impl Reservation {
    // 创建一个预定类
    pub fn new_pending(
        uid: impl Into<String>,
        rid: impl Into<String>,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: 0,
            user_id: uid.into(),
            resource_id: rid.into(),
            start: Some(convert_to_timestamp(&start.with_timezone(&Utc))),
            end: Some(convert_to_timestamp(&end.with_timezone(&Utc))),
            note: note.into(),
            status: ReservationStatus::Pending as i32,
        }
    }

    // 获取预定的起止时间段
    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }
}

impl Validate for Reservation {
    fn validate(&self) -> Result<(), Error> {
        // 校验用户ID
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(self.user_id.clone()));
        }
        // 校验资源ID
        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(self.resource_id.clone()));
        }
        // 校验预定时间段
        validate_range(self.start.as_ref(), self.end.as_ref())?;
        Ok(())
    }
}
