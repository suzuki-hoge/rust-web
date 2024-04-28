use chrono::Local;
use itertools::Itertools;
use r2d2_mysql::mysql::prelude::Queryable;

use crate::controller::ControllerResult;
use crate::database::mysql::Pool;

pub fn all(pool: &mut Pool) -> Result<ControllerResult, String> {
    let rows: Vec<(String, String)> = pool.select("select code, created from item").map_err(|e| e.to_string())?;

    let lines = rows.into_iter().map(|(code, created)| format!("code={}, created={}", code, created)).join(", ");

    Ok(ControllerResult::ok(format!("[{}]", lines)))
}

pub fn create(pool: &mut Pool, code: &String) -> Result<ControllerResult, String> {
    let created = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    pool.with_tx(|tx| {
        tx.exec_drop("insert item ( code, created ) values ( :code, :created )", vec![code, &created])
            .map_err(|e| e.to_string())
    })?;

    Ok(ControllerResult::ok(format!("code={}, created={}", code, &created)))
}
