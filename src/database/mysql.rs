use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use r2d2_mysql::mysql::{Conn, Opts, OptsBuilder, Transaction, TxOpts};
use r2d2_mysql::r2d2::ManageConnection;
use r2d2_mysql::MySqlConnectionManager;
use serde::{Deserialize, Serialize};

use crate::cache::memory::Memory;
use crate::cache::Cache;

pub struct Pool {
    manager: MySqlConnectionManager,
    cache: Memory,
}

impl Pool {
    pub fn new(host: &str, port: &str, user: &str, pass: &str, db: &str) -> Self {
        let opts = Opts::from_url(&format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db)).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        Self { manager: MySqlConnectionManager::new(builder), cache: Memory::new() }
    }

    fn connect(&mut self) -> Result<Conn, String> {
        self.manager.connect().map_err(|e| e.to_string())
    }

    pub fn select<R>(&mut self, query: &str) -> Result<Vec<R>, String>
    where
        R: FromRow + Serialize + for<'a> Deserialize<'a>,
    {
        if let Some(cached) = self.cache.get("item") {
            cached
        } else {
            let mut conn = self.connect()?;
            let found = conn.query(query).map_err(|e| e.to_string())?;

            self.cache.set("item", &found);

            Ok(found)
        }
    }

    pub fn with_tx<F>(&mut self, mut f: F) -> Result<(), String>
    where
        F: FnMut(&mut Transaction) -> Result<(), String>,
    {
        self.cache.clear();

        let mut conn = self.connect()?;
        let mut tx = conn.start_transaction(TxOpts::default()).map_err(|e| e.to_string())?;
        f(&mut tx)?;
        tx.commit().map_err(|e| e.to_string())
    }
}