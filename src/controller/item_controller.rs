use itertools::Itertools;
use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use r2d2_mysql::mysql::{Opts, OptsBuilder, TxOpts};
use r2d2_mysql::r2d2::ManageConnection;
use r2d2_mysql::MySqlConnectionManager;

use crate::controller::ControllerResult;

pub fn all() -> Result<ControllerResult, String> {
    let rows: Vec<(String, String)> = select("select code, created from item")?;
    let lines = rows.into_iter().map(|(code, created)| format!("code={}, created={}", code, created)).join(", ");

    Ok(ControllerResult::ok(format!("[{}]", lines)))
}

pub fn create(code: &String) -> Result<ControllerResult, String> {
    Ok(ControllerResult::ok(format!("id: {}, code: {}", 42, code)))
}

fn select<R: FromRow>(query: &str) -> Result<Vec<R>, String> {
    let opts = Opts::from_url("mysql://app:secret@localhost:13306/sales").map_err(|e| e.to_string())?;
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);

    let mut connection = manager.connect().map_err(|e| e.to_string())?;
    let mut transaction = connection.start_transaction(TxOpts::default()).map_err(|e| e.to_string())?;

    transaction.query(query).map_err(|e| e.to_string())
}
