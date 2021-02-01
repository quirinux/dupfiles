use clap::Clap;
use std::path::PathBuf;
use std::env;
use glob::glob;
use simple_logger::SimpleLogger;
use log::{LevelFilter, error, info, debug, trace};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;

mod processor;
mod view;
mod common;

#[derive(Debug)]
#[derive(Clap)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!(), about = clap::crate_description!())]
struct Opts {
    /// A level of verbosity, default: 0
    #[clap(short, long, parse(from_occurrences))]
    verbose: u32,
    /// Recurse down the path tree, default: false
    #[clap(short, long)]
    recursive: bool,
    /// Regex pattern to filter, default: .*
    #[clap(short, long)]
    pattern: Option<String>,
    /// Max depth recurtion level, implies recursive, default: 0
    #[clap(short, long)]
    depth: Option<usize>,
    /// Read from stdin, handful when piped, default: false
    #[clap(short = 'i', long)]
    stdin: bool,
    /// Output format [plain, json, file], default: plain
    #[clap(short, long)]
    format: Option<view::ViewType>,
    /// a shortcut to --format json
    #[clap(long)]
    json: bool,
    /// files only, a shortcut to --format file
    #[clap(long("files-only"))]
    files_only: bool,
    /// Path to look for duplicated files, default: current_dir
    path: Vec<PathBuf>,
}

fn index_files(mut depth: usize, path: PathBuf, pattern: &regex::Regex) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    if depth == 0 {
        return files;
    } else {
        depth -= 1;
    }
    trace!("depth:{:?}", depth);
    if path.is_dir() {
        if let Some(p) = path.to_str() {
            for entry in glob(format!("{}/{}", p, "*").as_str()).expect("could not read path") {
                match entry {
                    Ok(e) => files.append(&mut index_files(depth, e, pattern)),
                    Err(e) => error!("{:?}", e),
                }
            }
        }
    } else if path.is_file()  {
        if is_match(&path, pattern) {
            files.push(path);
        }
    }
    files
}

fn is_match(path: &PathBuf, pattern: &regex::Regex) -> bool {
    pattern.is_match(format!("{:?}", path).as_str())
}

fn main() {
    let mut opts: Opts = Opts::parse();
    let mut files: Vec<PathBuf> = Vec::new();
    let mut timer = common::timings::new(); 
    timer.start(common::timings::Timer::OverAll);

    // trying stdin, who knows when the plumber could pipe out
    let mut input_buffer = String::new();
    while opts.stdin {
        input_buffer.clear();
        match std::io::stdin().read_line(&mut input_buffer){
            Ok(n) => {
                if n > 0 {
                debug!("bytes read: {}", n);
                input_buffer.pop(); // removing newline traling char
                info!("adding {}", input_buffer);
                opts.path.push(PathBuf::from(input_buffer.clone()));
                } else {
                    info!("stdin is empty, moving on");
                    opts.stdin = false;
                }
            },
            Err(e) => error!("something went wrong => {}", e),
        }
    }

    // if no path was passed, setting to default
    if opts.path.len() == 0 {
        opts.path.push(PathBuf::from("."));
    };

    // recursive flag is just a short cut to
    // depth tending to infinty
    if opts.recursive && opts.depth.is_none() {
        opts.depth = Some(usize::MAX);
    } else if opts.depth.is_none() { // or else limit to working dir only
        opts.depth = Some(2);
    }

    let log_level = match opts.verbose {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace, // from 5 and above
    };

    SimpleLogger::new()
        .with_level(log_level)
        .init()
        .unwrap();

    debug!("{:?}", opts);

    // let's get rid off duplicated paths
    opts.path.sort_unstable();
    opts.path.dedup();
    debug!("path:{:?}", opts.path);

    // checking if it's Friday 13th
    if opts.json {
        opts.format = Some("json".parse().unwrap());
    } else if opts.files_only {
        opts.format = Some("file".parse().unwrap());
    }

    let pattern_string = match opts.pattern {
        Some(p) => p,
        None => ".*".to_string(),
    };
    let pattern = Regex::new(pattern_string.as_str()).unwrap();
    // time to list all files to be processed
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(100);
    bar.set_length(opts.path.len() as u64);
    bar.set_prefix("listing files");
    bar.set_style(ProgressStyle::default_spinner()
                  .template("{spinner} {pos}/{len} {prefix} - {msg}"));

    timer.start(common::timings::Timer::List);
    for entry in opts.path {
        info!("indexing files {:?} => dir:{:?} file:{:?}", entry.clone().canonicalize(), entry.is_dir(), entry.is_file());
        bar.set_message(entry.clone().canonicalize().unwrap().to_str().unwrap());
        bar.inc(1);
        let mut f = index_files(opts.depth.unwrap(), entry, &pattern);
        files.append(&mut f);
    }
    trace!("{:?}", files);
    files.sort_unstable();
    files.dedup();
    bar.finish_with_message(format!("found {}", files.len()).as_str());
    timer.stop(common::timings::Timer::List);

    // computing files
    info!("Computing files");
    timer.start(common::timings::Timer::Compute);
    let processor = processor::new(files);
    let files_tree = processor.compute_files();
    timer.stop(common::timings::Timer::Compute);
    debug!("{:?}", files_tree);


    timer.stop(common::timings::Timer::OverAll);
    let mut rs = common::resultset::new(timer, files_tree);
    rs.summarize();
    // showing results
    info!("showing results");
    let v = view::from_option(opts.format);
    v.show(rs);
}
