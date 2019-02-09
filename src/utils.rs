#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::prelude::DateTime;
use chrono::{offset::Local};
use std::time::{UNIX_EPOCH, Duration};

pub fn local_datetime_from_unix(time: u64) -> DateTime<Local> {
    DateTime::<Local>::from(UNIX_EPOCH + Duration::from_millis(time))
}
