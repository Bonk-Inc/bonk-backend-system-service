#[cfg(feature = "backend")]
extern crate diesel;

pub mod models;
#[cfg(feature = "backend")]
pub mod schema;