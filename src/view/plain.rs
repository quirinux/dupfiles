use indicatif::{HumanBytes};

use super::View;
use super::common::{timings, resultset};

pub struct Plain;

pub fn new() -> Plain {
    Plain{}
}

impl View for Plain {
    fn show(&self, rs: resultset::ResultSet) {
        for fs in rs.fileset {
            println!("{:?}:", fs.key);
            for file in fs.paths {
                println!("{}", file);
            };
            println!("duplicate amount size: {}\n", HumanBytes(fs.file_size as u64));
        };
        println!("Summary:");
        println!("Files amount: {}", rs.summary.total_files);
        println!("Duplicated files: {}", rs.summary.total_dup_files);
        println!("Duplicated file size: {}", HumanBytes(rs.summary.file_size_total as u64));
        println!("");
        println!("Timings:");
        println!("List files: {}", rs.stop_watch.duration_as_string(timings::Timer::List));
        println!("Compute files: {}", rs.stop_watch.duration_as_string(timings::Timer::Compute));
        println!("Overall: {}", rs.stop_watch.duration_as_string(timings::Timer::OverAll));
    }
}
