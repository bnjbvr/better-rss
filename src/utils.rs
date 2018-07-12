use std;
use rss;

pub type GenError = Box<std::error::Error>;
pub type GenResult<T> = Result<T, GenError>;
pub type Channel = (&'static str, &'static str, &'static str, Vec<rss::Item>);
