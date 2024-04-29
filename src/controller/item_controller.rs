use chrono::Local;
use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use r2d2_mysql::mysql::{from_row, FromRowError, Row};
use serde::{Deserialize, Serialize};

use crate::controller::ControllerResult;
use crate::database::mysql::Pool;

pub fn all(pool: &mut Pool) -> Result<ControllerResult, String> {
    let items: Vec<Item> = pool.select("select code, created from item").map_err(|e| e.to_string())?;

    Ok(ControllerResult::ok(items))
}

pub fn create(pool: &mut Pool, code: &str) -> Result<ControllerResult, String> {
    let item = Item { code: code.to_owned(), created: Local::now().format("%Y/%m/%d %H:%M:%S").to_string() };

    pool.with_tx(|tx| {
        tx.exec_drop("insert item ( code, created ) values ( :code, :created )", vec![&item.code, &item.created])
            .map_err(|e| e.to_string())
    })?;

    Ok(ControllerResult::ok(item))
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    code: String,
    created: String,
}

impl FromRow for Item {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        let (code, created) = from_row(row);
        Ok(Self { code, created })
    }
}
