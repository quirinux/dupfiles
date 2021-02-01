use super::View;
use super::common;

pub struct File;

pub fn new() -> File {
    File{}
}

impl View for File {
    fn show(&self, rs: common::resultset::ResultSet) {
        for fs in rs.fileset {
            for file in fs.paths {
                println!("{}", file);
            };
        };
    }
}
