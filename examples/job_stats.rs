use std::process;

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
}
