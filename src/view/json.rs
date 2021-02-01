use serde_json::json;

use super::View;
use super::common::{resultset};

pub struct JSON;

pub fn new() -> JSON {
    JSON{}
}

impl View for JSON {
    fn show(&self, rs: resultset::ResultSet) {
        let j = json!({
            "summary": rs.summary,
            "data": rs.fileset,
        });
        println!("{}", j);
    }
}
