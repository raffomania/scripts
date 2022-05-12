use std::{collections::HashMap, path::PathBuf};

use scripts::prelude::*;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    output: PathBuf,
    files: Vec<String>,
}
fn main() -> Result<()> {
    let args = Args::parse();

    let output = args
        .output
        .to_str()
        .expect("Could not convert output path to string???");

    let (with_handles, pure_files): (Vec<String>, Vec<String>) =
        args.files.into_iter().partition(|s| s.contains("="));

    let automatic_handle_letter_range = (b'A'..b'Z').map(char::from);

    let is_handle = regex::Regex::new("^[A-Z]+[0-9]*$")?;
    let automatic_handles: HashMap<String, String> = pure_files
        .iter()
        .filter(|s| !is_handle.is_match(s))
        .zip(automatic_handle_letter_range)
        .map(|(file, letter_handle)| (file.clone(), format!("AH{letter_handle}")))
        .collect();

    let formatted_automatic_handles: Vec<String> = automatic_handles
        .iter()
        .map(|(file, handle)| format!("{handle}={file}"))
        .collect();

    let transformed_files = pure_files.iter().map(|f| {
        if let Some(handle) = automatic_handles.get(f) {
            handle
        } else if is_handle.is_match(f) {
            f
        } else {
            panic!("Found file without automatic handle: {f}");
        }
    });

    let sh = xshell::Shell::new()?;
    sh.cmd("pdftk")
        .args(with_handles)
        .args(formatted_automatic_handles)
        .arg("cat")
        .args(transformed_files)
        .args(["output", output])
        .run()?;

    Ok(())
}
