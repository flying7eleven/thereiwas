use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub struct ThereIWasDatabaseConnection(Pool<ConnectionManager<PgConnection>>);

impl ThereIWasDatabaseConnection {
    #[inline(always)]
    pub fn get(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
        self.0.get()
    }
}

impl From<Pool<ConnectionManager<PgConnection>>> for ThereIWasDatabaseConnection {
    #[inline(always)]
    fn from(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        ThereIWasDatabaseConnection(pool)
    }
}
