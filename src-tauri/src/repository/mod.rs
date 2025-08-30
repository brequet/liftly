use crate::db::connection::DbPool;

pub mod exercise;
pub mod workout;

pub trait Repository: Sized {
    fn new(pool: DbPool) -> Self;
}
