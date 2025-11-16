# Rust Read Manga – Features

## Comprehensive Image Format Support
- JPG, PNG, WebP, and more  
- Single or multi-page manga  
- Automatic file detection  
- Graceful handling of corrupted or unsupported files  

## Video Conversion Engine
- Uses FFmpeg for video creation  
- Supports MP4, AVI, MKV (depending on FFmpeg build)  
- Fully configurable encoding pipeline  

## Flexible Video Configuration
- **Codecs**: `h264`, `h265`, `vp9`, `theora`  
- **Resolutions**: Custom width × height  
- **Bitrate**: Configurable (in kbps)  
- **Frame Rate**: Customizable (e.g. 24, 30, 60 fps)  

## Command-line Interface
You can pass parameters via command line or config file:
---
    -i, --input-dir <PATH>
    -o, --output-file <PATH>
    -c, --config <PATH>
    --codec <CODEC>
    --resolution <WxH>
    --fps <FPS>
    --bitrate <KBPS>

## Performance Optimizations
- Multithreading using **Rayon**  
- Asynchronous I/O powered by **Tokio**  
- Zero-cost abstractions and memory-safe operations  

## Advanced Capabilities
- Image processing (contrast, upscaling, watermark removal…)  
- Video effects (fade, zoom, slide, pan-and-scan…)  
- Audio synchronization  
- Batch processing (multiple manga / chapters)  

## Extensibility

- Plugin architecture  

- Custom scripting  

- API / Config file integration  



## Node Workflow System

- **Node-based workflows** for flexible processing pipelines.

- Similar to n8n, allowing you to connect different processing steps (nodes).

- **Nodes for**: Input, Image Processing, OCR, Timing, Video Encoding, and Output.

- Conditional routing and branching for complex workflows.



## Smart Page Duration Engine

- Automatically calculates page display duration based on **text content**.

- Uses OCR to determine the amount of text on a page.

- Configurable **reading speed** (words per minute or characters per second).

- Set minimum and maximum duration constraints for pages.



## Planned Features

- **Database Persistence**: Save workflow state and results to a database (e.g., PostgreSQL) for resumable workflows and analytics.

  