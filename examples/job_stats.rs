use std::io;
use std::process;

use rust_examples::job_stat_helper::{InputStat, JobStats};

fn main() {
    // fname was passed and exists
    let fname = match std::env::args().nth(1) {
        Some(f) => f,
        _ => {
            eprintln!("cargo run -- path_to_input.csv");
            process::exit(1i32)
        }
    };

    let mut csv_reader = match csv::Reader::from_path(&fname) {
        Ok(rdr) => rdr,
        Err(_) => {
            eprintln!("{fname} failed to open");
            process::exit(1i32);
        }
    };

    let input_stats = process_job_stat_csv(&mut csv_reader);
    let job_stats = JobStats::new(input_stats);

    println!("{}", job_stats);
}

fn process_job_stat_csv<R: io::Read>(rdr: &mut csv::Reader<R>) -> Vec<InputStat> {
    let mut job_input_stats = vec![];

    for line in rdr.deserialize() {
        job_input_stats.push(line.unwrap());


    }

    job_input_stats
}
