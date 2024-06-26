use std::sync::{Mutex, MutexGuard};

use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use r2d2_mysql::mysql::{Conn, Opts, OptsBuilder, Transaction, TxOpts};
use r2d2_mysql::r2d2::ManageConnection;
use r2d2_mysql::{mysql, MySqlConnectionManager};
use serde::{Deserialize, Serialize};

use crate::cache::memory::Memory;
use crate::cache::Cache;
use crate::logger::Logger;
use crate::LOGGER;

pub struct Pool {
    connections: Vec<Mutex<Conn>>,
    cache: Memory,
}

impl Pool {
    pub fn new(host: &str, port: &str, user: &str, pass: &str, db: &str, limit: u8) -> Self {
        let opts = Opts::from_url(&format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db)).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MySqlConnectionManager::new(builder);

        let mut connections = Vec::with_capacity(limit as usize);
        for _ in 0..limit {
            connections.push(Mutex::new(manager.connect().unwrap()));
            LOGGER.info("create connection");
        }

        LOGGER.info(format!("init {} connections", limit));

        Self { connections, cache: Memory::new() }
    }

    pub fn select<R>(&self, query: &str, caching: bool) -> mysql::error::Result<Vec<R>>
    where
        R: FromRow + Serialize + for<'a> Deserialize<'a>,
    {
        if caching {
            if let Some(cached) = self.cache.get("item") {
                LOGGER.info("find from cache");
                Ok(cached)
            } else {
                let found = self.no_cache_select(query)?;

                self.cache.set("item", &found);
                LOGGER.info("save into cache");

                Ok(found)
            }
        } else {
            self.no_cache_select(query)
        }
    }

    fn no_cache_select<R>(&self, query: &str) -> mysql::error::Result<Vec<R>>
    where
        R: FromRow + Serialize + for<'a> Deserialize<'a>,
    {
        let mut conn = self.get_conn();
        LOGGER.info(format!("find from database [connection {}]", conn.connection_id()));

        let found = conn.query(query)?;

        Ok(found)
    }

    pub fn with_tx<F>(&self, mut f: F, caching: bool) -> mysql::error::Result<()>
    where
        F: FnMut(&mut Transaction) -> mysql::error::Result<()>,
    {
        if caching {
            self.cache.clear();
        }

        let mut conn = self.get_conn();
        LOGGER.info(format!("save into database [connection {}]", conn.connection_id()));

        let mut tx = conn.start_transaction(TxOpts::default())?;
        f(&mut tx)?;
        tx.commit()
    }

    fn get_conn(&self) -> MutexGuard<Conn> {
        loop {
            for m in &self.connections {
                if let Ok(mg) = m.try_lock() {
                    return mg;
                }
            }
        }
    }
}
