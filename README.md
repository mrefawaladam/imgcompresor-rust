# Image Compressor

A high-performance, production-ready image compression tool built with Rust. Compress images up to **95% smaller** while maintaining visual quality, perfect for reducing storage costs and improving website performance.

## Why This Matters

- **Save Storage Costs**: Reduce image file sizes by 70-95% without noticeable quality loss
- **Faster Websites**: Smaller images load faster, improving user experience and SEO rankings
- **Batch Processing**: Compress entire folders of images automatically
- **Production Ready**: Built with Rust for maximum performance and reliability

## Key Features

✅ **Smart Compression** - Automatically adjusts quality based on file size  
✅ **Batch Processing** - Compress entire directories with one command  
✅ **Multi-threaded** - Uses all CPU cores for maximum speed  
✅ **Format Support** - JPEG, PNG, WebP, TIFF, BMP  
✅ **Zero Quality Loss Options** - Choose your preferred balance between size and quality  
✅ **Progress Tracking** - Real-time progress bar shows compression status  

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/mrefawaladam/imgcompresor-rust
cd imgcompressor

# Build the project
cargo build --release
```

### Basic Usage

**Compress a single image:**
```bash
cargo run --release -- --input photo.jpg --output compressed.jpg --quality 30
```

**Compress an entire folder:**
```bash
cargo run --release -- --input ./photos/ --output ./compressed/ --quality 30
```

## Performance

- **Speed**: Compresses 3.6 MB image in ~1-2 seconds (release mode)
- **Efficiency**: 2-3x faster than debug mode with optimized builds
- **Scalability**: Processes multiple images in parallel using all available CPU cores

## Use Cases

- **E-commerce**: Compress product images to reduce bandwidth costs
- **Content Management**: Optimize images before uploading to CMS
- **Mobile Apps**: Reduce app size by compressing assets
- **Web Development**: Prepare images for faster page loads
- **Backup Storage**: Compress image archives to save storage space

## Quality Settings

- **10-20**: Maximum compression, smaller files (best for thumbnails)
- **25-40**: Balanced compression (recommended for most use cases)
- **50-80**: Higher quality, larger files (for print or high-resolution needs)

## Requirements

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- macOS, Linux, or Windows

## Example Output

```
Ditemukan 5 file — total ukuran 12.45 MB
████████████████████████████████████████ 5/5 Selesai

Summary:
  File diproses    : 5
  Total sebelum    : 12.45 MB
  Total sesudah    : 3.21 MB
  Total dihemat    : 9.24 MB (74% reduction)
  Durasi           : 2.34s
```

## Technical Details

Built with:
- **Rust** - Memory-safe systems programming language
- **mozjpeg** - Industry-leading JPEG compression library
- **rayon** - Data parallelism library for multi-threading
- **clap** - Command-line argument parser

## License

[Add your license here]

---

**Built for performance. Designed for simplicity.**

