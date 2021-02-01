use std::path::PathBuf;
use md5;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use log::{error, debug, trace};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, ParallelProgressIterator};

const BUFFER_SIZE: usize = 65535;

pub struct Processor {
    files: Vec<PathBuf>,
}

pub fn new(files: Vec<PathBuf>) -> Processor {
    Processor{
        files,
    }        
}

impl Processor {
    fn calculate_md5(&self, file: PathBuf) -> std::io::Result<md5::Digest> {
        let mut f = File::open(file)?;
        let mut md5_context = md5::Context::new();
        let mut buffer = [0; BUFFER_SIZE];

        loop {
            if f.read(&mut buffer)? > 0 {
                md5_context.consume(&buffer);
            } else {
                break;
            }
        }

        Ok(md5_context.compute())
    }

    fn run_parallel(&self) -> Vec<(PathBuf, Result<md5::Digest, std::io::Error>)> {
        let count = self.files.len() as u64;
        let bar = ProgressBar::new(count);
        bar.set_prefix("processing files");
        bar.set_style(ProgressStyle::default_bar()
            .template("{prefix} - [{elapsed}] {wide_bar} {percent}% {pos}/{len} {eta}"));

        let digests: Vec<_> = self.files.par_iter()
           .progress_with(bar)
           .map(|file| 
                  (file.clone(), self.calculate_md5(file.to_path_buf()))
              )
           .collect();
       digests
    }

    pub fn compute_files(&self) -> HashMap<md5::Digest, Vec<PathBuf>> {
        let mut files_tree: HashMap<md5::Digest, Vec<PathBuf>> = HashMap::new();
        let digests = self.run_parallel();
        trace!("{:?}", digests);
        for (file, digest) in digests {
            match digest {
                Ok(d) => {
                    if files_tree.contains_key(&d) {
                        debug!("{:?} found", d);
                        if let Some(idx) = files_tree.get_mut(&d) {
                            idx.push(file);
                        }
                    } else {
                        files_tree.insert(
                                d,
                                vec![file],
                            );
                    }
                },
                Err(e) => error!("{:?}", e),
            }
        }
        files_tree
    }
}
