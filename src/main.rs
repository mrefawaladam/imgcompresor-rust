mod args;
mod compressor;
mod file_collector;
mod stats;

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use args::Args;
use compressor::process_one_file;
use file_collector::{collect_input_files, prepare_output_dir};
use stats::FileStat;

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(n) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
            .context("Gagal setup thread pool")?;
    }

    let start = Instant::now();

    let files = collect_input_files(&args.input)?;
    if files.is_empty() {
        anyhow::bail!("Tidak ada file gambar ditemukan di input path");
    }

    let total_input_size: u64 = files
        .iter()
        .map(|p| fs::metadata(p).map(|m| m.len()).unwrap_or(0))
        .sum();

    println!(
        "Ditemukan {} file — total ukuran {:.2} MB",
        files.len(),
        total_input_size as f64 / 1024.0 / 1024.0
    );

    prepare_output_dir(&args.output)?;

    let pb = Arc::new(Mutex::new(ProgressBar::new(files.len() as u64)));
    pb.lock().unwrap().set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
        )
        .unwrap()
        .progress_chars("█▌ "),
    );

    let stats: Vec<FileStat> = files
        .par_iter()
        .map(|path| {
            let t0 = Instant::now();

            let res = process_one_file(path, &args);

            let dur = t0.elapsed().as_millis();

            let pb_clone = Arc::clone(&pb);
            let stat = match &res {
                Ok((before, after)) => {
                    FileStat::success(path.clone(), *before, *after, dur)
                }
                Err(e) => FileStat::error(path.clone(), format!("{:?}", e), dur),
            };
            let msg = match &stat.error {
                Some(err) => format!("error: {}", short_str(err, 50)),
                None => format!(
                    "hemat {:.2} KB",
                    stat.saved_bytes() as f64 / 1024.0
                ),
            };
            pb_clone.lock().unwrap().inc(1);
            pb_clone.lock().unwrap().set_message(msg);

            stat
        })
        .collect::<Vec<_>>();

    pb.lock().unwrap().finish_with_message("Selesai");

    let total_before: u64 = stats.iter().map(|s| s.before).sum();
    let total_after: u64 = stats.iter().map(|s| s.after).sum();
    let total_saved = total_before.saturating_sub(total_after);

    println!();
    println!("Summary:");
    println!("  File diproses    : {}", stats.len());
    println!("  Total sebelum    : {:.2} MB", total_before as f64 / 1024.0 / 1024.0);
    println!("  Total sesudah    : {:.2} MB", total_after as f64 / 1024.0 / 1024.0);
    println!("  Total dihemat    : {:.2} MB", total_saved as f64 / 1024.0 / 1024.0);
    println!("  Durasi           : {:.2}s", start.elapsed().as_secs_f64());

    let errors: Vec<&FileStat> = stats.iter().filter(|s| s.error.is_some()).collect();
    if !errors.is_empty() {
        println!();
        println!("Error ({}):", errors.len());
        for e in errors {
            println!(" - {} => {}", e.path.display(), e.error.as_ref().unwrap());
        }
    }

    Ok(())
}

fn short_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}
