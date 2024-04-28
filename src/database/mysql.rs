use r2d2_mysql::mysql::{Conn, Opts, OptsBuilder};
use r2d2_mysql::MySqlConnectionManager;
use r2d2_mysql::r2d2::ManageConnection;

pub struct Pool {
    manager: MySqlConnectionManager,
}

impl Pool {
    pub fn new(host: &str, port: &str, user: &str, pass: &str, db: &str) -> Self {
        let opts = Opts::from_url(&format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db)).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        Self { manager: MySqlConnectionManager::new(builder) }
    }

    pub fn connect(&mut self) -> Result<Conn, String> {
        self.manager.connect().map_err(|e| e.to_string())
    }
}
