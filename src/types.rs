use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use warp::reject::Reject;

#[derive(Debug, Clone, Copy)]
pub enum SourceName {
    Safari,
    Firefox,
    Chrome,
}

#[derive(Serialize)]
pub struct VisitDetail {
    pub url: String,
    pub title: String,
    // unix_epoch_ms
    pub visit_time: i64,
    pub visit_type: i64,
}

#[derive(Debug)]
pub struct DailyCount {
    // unix_epoch_ms
    pub day: i64,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
pub struct TimeRange {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Debug)]
pub struct ServerError {
    pub e: String,
}

impl From<Error> for ServerError {
    fn from(err: Error) -> Self {
        Self { e: err.to_string() }
    }
}

impl Reject for ServerError {}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}