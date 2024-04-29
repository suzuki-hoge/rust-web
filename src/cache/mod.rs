pub mod memory;

use serde::{Deserialize, Serialize};

pub trait Cache {
    fn get<T: for<'a> Deserialize<'a>>(&self, key: &str) -> Option<T>;

    fn set<S: Serialize>(&self, key: &str, s: &S);

    fn clear(&self);
}
