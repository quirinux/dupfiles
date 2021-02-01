use std::collections::HashMap;
use std::path::PathBuf;
use md5;
use log::{error, debug, trace};
use serde::{Deserialize, Serialize};
use super::timings;

#[derive(Serialize, Deserialize)]
pub struct FileSet {
    pub key: String,
    pub paths: Vec<String>,
    pub file_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    pub file_size_total: usize,
    pub total_dup_files: usize,
    pub total_files: usize,
}

pub struct ResultSet {
    pub stop_watch: timings::StopWatch,
    files_tree: HashMap<md5::Digest, Vec<PathBuf>>,
    pub fileset: Vec<FileSet>,
    pub summary: Summary,
}

pub fn new(stop_watch: timings::StopWatch, files_tree: HashMap<md5::Digest, Vec<PathBuf>>) -> ResultSet {
    ResultSet {
        stop_watch: stop_watch,
        files_tree: files_tree,
        fileset: vec![],
        summary: Summary {
            file_size_total: 0,
            total_dup_files: 0,
            total_files: 0,
        },
    }
}

impl ResultSet {
    pub fn summarize(&mut self) {
        for (key, val)  in self.files_tree.iter() {
            self.summary.total_files += 1;
            if val.len() > 1 {
                self.summary.total_dup_files += val.len() - 1;
                debug!("summarizing => {:?}:", key);
                let mut fs = FileSet{key: format!("{:?}", key), paths: vec![], file_size: 0};
                for item in val {
                    self.summary.total_files += 1;
                    if fs.file_size == 0 {
                        match std::fs::metadata(item) {
                            Ok(f) => fs.file_size = f.len() as usize,
                            Err(e) => error!("{:?}", e),
                        }
                    }
                    trace!("{}", item.to_string_lossy());
                    fs.paths.push(format!("{}", item.to_string_lossy()));
                }
                self.summary.file_size_total += fs.file_size as usize * (val.len() - 1);
                self.fileset.push(fs);
            }
        }
    }
}
