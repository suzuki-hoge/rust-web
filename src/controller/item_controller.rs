use std::sync::Arc;
use std::thread;
use std::time::Duration;

use chrono::Local;
use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use r2d2_mysql::mysql::{from_row, FromRowError, Row};
use serde::{Deserialize, Serialize};

use crate::controller::ControllerResult;
use crate::database::mysql::Pool;

pub fn read(pool: Arc<Pool>, caching: bool) -> Result<ControllerResult, String> {
    let items: Vec<Item> = pool.select("select code, at from item", caching).map_err(|e| e.to_string())?;

    Ok(ControllerResult::ok(items))
}

pub fn write(pool: Arc<Pool>, caching: bool, code: Result<&String, String>) -> Result<ControllerResult, String> {
    match code {
        Ok(code) => {
            let item = Item { code: code.to_owned(), at: Local::now().format("%Y/%m/%d %H:%M:%S").to_string() };

            pool.with_tx(
                |tx| tx.exec_drop("insert item ( code, at ) values ( :code, :at )", vec![&item.code, &item.at]),
                caching,
            )
            .map_err(|e| e.to_string())?;

            Ok(ControllerResult::ok(item))
        }
        Err(e) => Ok(ControllerResult::bad_request(e)),
    }
}

pub fn block(pool: Arc<Pool>) -> Result<ControllerResult, String> {
    pool.with_tx(
        |_| {
            thread::sleep(Duration::from_secs(3));
            Ok(())
        },
        false,
    )
    .map_err(|e| e.to_string())?;

    Ok(ControllerResult::ok("connection slept 3 seconds"))
}

pub fn thread_sleep(time: u64) -> Result<ControllerResult, String> {
    thread::sleep(Duration::from_secs(time));

    Ok(ControllerResult::ok(format!("thread slept {} seconds", time)))
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    code: String,
    at: String,
}

impl FromRow for Item {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        let (code, at) = from_row(row);
        Ok(Self { code, at })
    }
}
