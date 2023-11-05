use crate::{Reservation, ReservationStatus};
use chrono::{DateTime, FixedOffset};

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
            start: Some(),
            end,
            note: note.into(),
            status: ReservationStatus::Pending as i32,
        }
    }
}
