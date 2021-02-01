use std::str::FromStr;
use std::num::ParseIntError;
use super::common;

pub mod plain;
pub mod json;
pub mod file;

pub trait View {
    // fn show(&self, files_tree: HashMap<md5::Digest, Vec<PathBuf>>, timer: common::timings::StopWatch);
    fn show(&self, rs: common::resultset::ResultSet);
}

#[derive(Debug, PartialEq)]
pub enum ViewType {
    Plain,
    JSON,
    File,
}

impl FromStr for ViewType {
    type Err = ParseIntError;

    fn from_str(vt: &str) -> Result<Self, Self::Err> {
        Ok(match vt {
            "plain" => ViewType::Plain, 
            "json" => ViewType::JSON,
            "file" => ViewType::File,
            _ => ViewType::Plain,
        })
    }
}

pub fn new(vt: ViewType) -> Box<dyn View> {
    match vt {
        ViewType::Plain => Box::new(plain::new()),
        ViewType::JSON => Box::new(json::new()),
        ViewType::File => Box::new(file::new()),
    }
}

pub fn from_option(vt: Option<ViewType>) -> Box<dyn View> {
    match vt {
        Some(v) => new(v),
        None => new(ViewType::Plain),
    }
}



