use anyhow::{Context, Result};
use image::{DynamicImage, ImageReader};
use mozjpeg::Compress;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::args::Args;

fn calculate_quality(base_quality: u8, file_size_bytes: u64) -> u8 {
    let size_mb = (file_size_bytes as f64) / 1024.0 / 1024.0;
    let mut quality = base_quality as i32;

    if size_mb > 20.0 {
        quality -= 20;
    } else if size_mb > 5.0 {
        quality -= 10;
    }

    quality.clamp(5, 100) as u8
}

pub fn process_one_file(path: &PathBuf, args: &Args) -> Result<(u64, u64)> {
    let meta = fs::metadata(path)?;
    let before_size = meta.len();

    let quality = calculate_quality(args.quality, before_size);

    let dyn_img = ImageReader::open(path)
        .with_context(|| format!("Gagal membuka file: {}", path.display()))?
        .with_guessed_format()?
        .decode()
        .with_context(|| format!("Gagal decode gambar: {}", path.display()))?;

    let output_path = if args.input.is_file() {
        args.output.clone()
    } else {
        let rel = path.strip_prefix(&args.input).unwrap_or(path);
        let mut out = args.output.join(rel);
        out.set_extension("jpg");
        out
    };

    if output_path.exists() && !args.overwrite {
        let existing_size = fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0);
        return Ok((before_size, existing_size));
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let (after_size, _) = encode_with_mozjpeg(&dyn_img, &output_path, quality)
        .with_context(|| format!("Gagal encode gambar: {}", path.display()))?;

    Ok((before_size, after_size))
}

fn encode_with_mozjpeg(img: &DynamicImage, out_path: &Path, quality: u8) -> Result<(u64, ())> {
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    let mut comp = Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    comp.set_size(width as usize, height as usize);
    comp.set_quality(quality as f32);
    comp.set_optimize_scans(false);

    let outfile = File::create(out_path)?;
    let mut writer = BufWriter::new(outfile);

    let mut started = comp.start_compress(&mut writer)?;
    started.write_scanlines(&rgb)?;
    started.finish()?;

    writer.flush()?;

    let after = fs::metadata(out_path)?.len();
    Ok((after, ()))
}
