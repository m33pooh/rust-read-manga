# rust-read-manga.md

This document provides information about the `rust-read-manga` project, which is designed to convert manga images into video format. The project leverages Rust for performance-critical image processing and video encoding.

## Project Overview

`rust-read-manga` is a Rust-based application that:
- Processes manga image files (JPG, PNG, etc.)
- Converts them into a video format (MP4, AVI, etc.)
- Supports various video codecs and resolutions
- Utilizes efficient memory management features of Rust

## Installation

To install the `rust-read-manga` CLI tool:

```bash
cargo install --git https://github.com/username/rust-read-manga.git
```

## Basic Usage

```bash
# Convert a single manga chapter from a directory
rust-read-manga convert /path/to/manga/chapter --output video.mp4

# Convert with specific video settings
rust-read-manga convert --fps 24 --resolution 1920x1080 --bitrate 8M /path/to/manga

# Convert with transition effects
rust-read-manga convert --transitions --fade-duration 500 /path/to/manga
```

## Configuration

Create a `RustReadManga.toml` configuration file in your project root:

```toml
[output]
filename = "manga_video.mp4"
format = "mp4"
quality = "high"

[video]
fps = 24
resolution = "1920x1080"
bitrate = "8M"
codec = "h264"

[processing]
concurrent_workers = 4
cache_images = true
```
