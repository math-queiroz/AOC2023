use itertools::Itertools;
use std::{error::Error, fs, process::Command};

// From https://github.com/AxlLind/AdventOfCode2022/blob/main/src/main.rs
fn main() -> Result<(), Box<dyn Error>> {
    println!("Running AdventOfCode2023 for each day:\n");

    let days = fs::read_dir("./src/bin")?
        .filter_map(|e| e.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .filter(|bin| bin.parse::<usize>().is_ok())
        .sorted()
        .collect::<Vec<_>>();

    for day in &days {
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", day])
            .output()?;
        let out = String::from_utf8(cmd.stdout)?;
        println!("Day {}\n{}", day, out);
    }

    Ok(())
}
