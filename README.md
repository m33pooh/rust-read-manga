# Rust Read Manga

**Rust Read Manga** is a high-performance Rust application that converts manga image files into video format. It leverages Rust’s memory safety, concurrency, and speed to create fast, reliable video adaptations of manga content.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Node Workflow System](#node-workflow-system)
- [Smart Page Duration Engine](#smart-page-duration-engine)
- [Database Persistence](#database-persistence)
- [Contributing](#contributing)

---

## Features

### Comprehensive Image Support
- Load manga images in JPG, PNG, WebP, etc.
- Handle single or multi-page manga
- Automatic file detection
- Graceful handling of corrupted files

### Video Conversion
- Converts image sequences into video using **FFmpeg**
- Supports MP4, AVI, MKV containers
- Adjustable output settings (resolution, codec, bitrate, frame rate)

### Advanced Video Features
- Transitions: fade, zoom, slide
- Pan-and-scan motion
- Audio synchronization
- Batch processing multiple manga series

### Image Processing
- Auto-contrast and upscaling
- Watermark removal
- Batch operations

### Performance Optimizations
- Zero-cost abstractions
- Multithreading with Rayon
- Async I/O with Tokio
- Memory-safe operations

### Extensibility
- Plugin architecture
- Custom scripts
- Integration API
- Config file support

---

## Installation

1. Install Rust (stable) via [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/rust-read-manga.git
   cd rust-read-manga
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

---

## Usage

You can configure Rust Read Manga via `config.toml` or command-line arguments:

```bash
rust-read-manga -i /path/to/images -o output.mp4 --codec h264 --resolution 1920x1080 --fps 30 --bitrate 5000
```

**Available CLI arguments:**

| Argument | Description |
|----------|-------------|
| `-i, --input-dir <PATH>` | Input image directory |
| `-o, --output-file <PATH>` | Output video file |
| `-c, --config <PATH>` | Optional custom config TOML |
| `--codec <CODEC>` | Video codec (`h264`, `h265`, `vp9`, `theora`) |
| `--resolution <WxH>` | Video resolution |
| `--fps <FPS>` | Frames per second |
| `--bitrate <KBPS>` | Video bitrate in kbps |

---

## Node Workflow System

Rust Read Manga supports a **node-based workflow system**, similar to n8n, to flexibly process images, text, timing, and video steps.

### Node Categories
- Input Nodes: Load images
- Image Nodes: Preprocessing, filters, upscaling
- Text Nodes: OCR, text density calculation
- Timing Nodes: Page duration calculation
- Video Nodes: Transitions, encoding, audio sync
- Output Nodes: Export video, logging
- Logic Nodes: Conditional routing, merging
- Utility Nodes: Counters, timers, random values

> See [NODES.md](docs/NODES.md) for full node documentation.

---

## Smart Page Duration Engine

Automatically calculates **how long each manga page remains on screen** using:

- OCR-based text extraction
- Word/character count
- Configurable reading speed (WPM/CPM)
- Minimum and maximum duration constraints
- Dynamic timeline generation

> See [DURATION_ENGINE.md](docs/DURATION_ENGINE.md) for details.

---

## Database Persistence

Rust Read Manga can **save intermediate workflow steps to PostgreSQL**, allowing:

- Workflow resuming
- Debugging and auditing
- Analytics on node outputs

> See [SAVE_STEPS_POSTGRES.md](docs/SAVE_STEPS_POSTGRES.md) for
