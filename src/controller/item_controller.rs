use chrono::Local;
use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use r2d2_mysql::mysql::{from_row, FromRowError, Row};
use serde::{Deserialize, Serialize};

use crate::controller::ControllerResult;
use crate::database::mysql::Pool;

pub fn all(pool: &mut Pool) -> Result<ControllerResult, String> {
    let items: Vec<Item> = pool.select("select code, at from item").map_err(|e| e.to_string())?;

    Ok(ControllerResult::ok(items))
}

pub fn create(pool: &mut Pool, code: &str) -> Result<ControllerResult, String> {
    let item = Item { code: code.to_owned(), at: Local::now().format("%Y/%m/%d %H:%M:%S").to_string() };

    pool.with_tx(|tx| {
        tx.exec_drop("insert item ( code, at ) values ( :code, :at )", vec![&item.code, &item.at])
            .map_err(|e| e.to_string())
    })?;

    Ok(ControllerResult::ok(item))
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
