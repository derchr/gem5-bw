use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read},
    process::ExitCode,
};

const WINDOW_SIZE: u32 = 100000;

struct Ticks(u64);
struct Nanoseconds(u64);

impl From<Ticks> for Nanoseconds {
    fn from(ticks: Ticks) -> Self {
        Nanoseconds(ticks.0 / 1000)
    }
}

fn main() -> ExitCode {
    let filepath = match std::env::args().nth(1) {
        Some(filename) => filename,
        None => {
            eprintln!("Need to specify trace file.");
            return ExitCode::FAILURE;
        }
    };

    let trace = match std::fs::File::open(filepath) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Cannot open trace file.");
            return ExitCode::FAILURE;
        }
    };

    let read_buf = BufReader::new(trace);

    let transfers: Vec<Nanoseconds> = read_buf
        .lines()
        .filter(|line| line.as_ref().unwrap().contains("Responding to Address"))
        .map(|line| line.unwrap().split_once(":").unwrap().0.trim().to_owned())
        .map(|line| Ticks(line.parse::<u64>().unwrap()).into())
        .collect();

    let mut windows: BTreeMap<u64, u64> = BTreeMap::new();

    for transfer in transfers {
        let window = transfer.0 / WINDOW_SIZE as u64;

        *windows.entry(window).or_insert(1) += 1;
    }

    for window in windows {
        let ns = window.0 * WINDOW_SIZE as u64;
        let bytes = window.1 * 64;

        let bw = bytes as f64 / (1f64 / (1_000_000_000 / WINDOW_SIZE) as f64);
        println!("{ns},{bw}");
    }

    ExitCode::SUCCESS
}
