use indicatif::ProgressBar;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    str::FromStr,
};
use tar::Archive;
use zstd::stream::read::Decoder;

mod analysis;
use analysis::OthelloAnalysis;

const BOARD_SIZE: i8 = 64;
const DEPTH_LIMIT: u8 = 36;
const STRENGTH_LIMIT: u8 = 100;
const TOTAL_FILES: u64 = 2587;

fn main() -> io::Result<()> {
    let file = File::open("knowledge_archive.tar.zst")?;
    let zstd_decoder = Decoder::new(file)?;
    let buf_reader = BufReader::new(zstd_decoder);
    let mut archive = Archive::new(buf_reader);

    let pb = ProgressBar::new(TOTAL_FILES);

    let filename_regex = Regex::new(r"^knowledge_archive/knowledge_([OX-]{64})\.csv$").unwrap();

    for entry_result in archive.entries()? {
        let entry = entry_result?;

        // Validate filename
        let filename = entry.path().unwrap().to_string_lossy().into_owned();
        if let Some(captures) = filename_regex.captures(&filename) {
            let board_state = captures.get(1).unwrap();
            if board_state.as_str().chars().filter(|&c| c == '-').count() != 50 {
                eprintln!("Error: Board state does not have exactly 50 '-' characters: {filename}");
                continue;
            }
        } else {
            eprintln!("Error: Filename does not match expected pattern: {filename}");
            continue;
        }

        let reader = BufReader::new(entry);

        for line_result in reader.lines() {
            let line = line_result?;
            match OthelloAnalysis::from_str(&line) {
                Ok(data) => {
                    assert!((data.board[0] | data.board[1]).count_zeros() == DEPTH_LIMIT as u32);
                    assert!(data.settings[0] <= DEPTH_LIMIT);
                    assert!(data.settings[1] <= STRENGTH_LIMIT);
                    assert!(data.bounds[0].abs() <= BOARD_SIZE);
                    assert!(data.bounds[1].abs() <= BOARD_SIZE);
                    assert!(data.bounds[0] % 2 == data.bounds[1] % 2);
                    assert!(data.bounds[0] <= data.bounds[1]);
                    assert!(data.nodes.abs() < 1 << 47);
                    assert!(
                        data.bounds[0] == data.bounds[1]
                            || data.bounds[1] < 0
                            || data.bounds[0] > 0
                    );

                    if data.settings[0] == DEPTH_LIMIT && data.settings[1] == STRENGTH_LIMIT
                    // && data.bounds[0] == data.bounds[1]
                    {
                        if let Err(e) = std::io::stdout().write_all(&data.to_bytes()) {
                            eprintln!("Error writing to stdout: {e}");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}\n{line}\n{filename}");
                }
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message("Done!");
    Ok(())
}
