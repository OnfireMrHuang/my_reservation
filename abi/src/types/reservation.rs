use std::ops::Bound;

use super::validate_range;
use crate::{
    convert_to_timestamp, get_timespan, Error, Reservation, ReservationStatus, RsvpStatus, Validate,
};
use chrono::{DateTime, FixedOffset, Utc};
use sqlx::{
    postgres::{types::PgRange, PgRow},
    FromRow, Row,
};

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

// 实现从sqlx::Row转换为abi::Reservation的方法
impl FromRow<'_, PgRow> for Reservation {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.get("id");
        let range: PgRange<DateTime<Utc>> = row.get("timespan");
        let range: NativeRange<DateTime<Utc>> = range.into();

        // 在真实的世界中，预定总是存在边界的
        assert!(range.start.is_some());
        assert!(range.end.is_some());

        let start = range.start.unwrap();
        let end = range.end.unwrap();

        let status: RsvpStatus = row.get("status");

        Ok(Self {
            id,
            user_id: row.get("user_id"),
            resource_id: row.get("resource_id"),
            start: Some(convert_to_timestamp(&start)),
            end: Some(convert_to_timestamp(&end)),
            note: row.get("note"),
            status: ReservationStatus::from(status) as i32,
        })
    }
}

// 定义一个原生范围类
struct NativeRange<T> {
    start: Option<T>,
    end: Option<T>,
}

// 实现从PG的范围类转换为原生范围类的方法
impl<T> From<PgRange<T>> for NativeRange<T> {
    fn from(value: PgRange<T>) -> Self {
        let f = |b: Bound<T>| match b {
            Bound::Included(t) => Some(t),
            Bound::Excluded(t) => Some(t),
            Bound::Unbounded => None,
        };
        let start = f(value.start);
        let end = f(value.end);

        Self { start, end }
    }
}
