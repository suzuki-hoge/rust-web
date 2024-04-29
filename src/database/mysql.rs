use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use r2d2_mysql::mysql::{Conn, Opts, OptsBuilder, Transaction, TxOpts};
use r2d2_mysql::r2d2::ManageConnection;
use r2d2_mysql::{mysql, MySqlConnectionManager};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

use crate::cache::memory::Memory;
use crate::cache::Cache;
use crate::logger::Logger;
use crate::LOGGER;

pub struct Pool {
    conn: Mutex<Conn>,
    cache: Memory,
}

impl Pool {
    pub fn new(host: &str, port: &str, user: &str, pass: &str, db: &str) -> Self {
        let opts = Opts::from_url(&format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db)).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MySqlConnectionManager::new(builder);

        let conn = Mutex::new(manager.connect().unwrap());

        Self { conn, cache: Memory::new() }
    }

    pub fn select<R>(&self, query: &str) -> mysql::error::Result<Vec<R>>
    where
        R: FromRow + Serialize + for<'a> Deserialize<'a>,
    {
        if let Some(cached) = self.cache.get("item") {
            LOGGER.info("from cache");

            Ok(cached)
        } else {
            let mut conn = self.get_conn();
            let found = conn.query(query)?;

            self.cache.set("item", &found);

            LOGGER.info(format!("from database [ connection = {} ]", conn.connection_id()));

            Ok(found)
        }
    }

    pub fn with_tx<F>(&self, mut f: F) -> mysql::error::Result<()>
    where
        F: FnMut(&mut Transaction) -> mysql::error::Result<()>,
    {
        self.cache.clear();

        let mut conn = self.get_conn();
        let mut tx = conn.start_transaction(TxOpts::default())?;
        f(&mut tx)?;
        tx.commit()
    }

    fn get_conn(&self) -> MutexGuard<Conn> {
        self.conn.lock().unwrap()
    }
}
