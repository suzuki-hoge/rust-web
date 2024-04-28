use chrono::Local;
use itertools::Itertools;
use r2d2_mysql::mysql::{Conn, TxOpts};
use r2d2_mysql::mysql::prelude::Queryable;

use crate::controller::ControllerResult;

pub fn all(conn: &mut Conn) -> Result<ControllerResult, String> {
    let rows: Vec<(String, String)> = conn.query("select code, created from item").map_err(|e| e.to_string())?;

    let lines = rows.into_iter().map(|(code, created)| format!("code={}, created={}", code, created)).join(", ");

    Ok(ControllerResult::ok(format!("[{}]", lines)))
}

pub fn create(conn: &mut Conn, code: &String) -> Result<ControllerResult, String> {
    let created = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    let mut tx = conn.start_transaction(TxOpts::default()).map_err(|e| e.to_string())?;
    tx.exec_drop("insert item ( code, created ) values ( :code, :created )", vec![code, &created]).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;

    Ok(ControllerResult::ok(format!("code={}, created={}", code, &created)))
}
