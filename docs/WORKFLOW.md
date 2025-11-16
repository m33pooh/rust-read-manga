# Rust Read Manga – Workflow System

Rust Read Manga provides a **node-based workflow system**, loosely inspired by platforms like **n8n**, for configuring and automating the manga-to-video pipeline.

## Key Concepts

- **Nodes**: Modular units of work (e.g. OCR, encode, filter)  
- **Parameters**: Each node has configurable parameters  
- **Dynamic Linking**: Nodes can pass values to downstream nodes  
- **Conditional Logic**: If / Switch nodes for branching  
- **Reusable Templates**: Workflows can be saved as JSON or TOML  
- **Override Hierarchy**:  
  - CLI overrides workflow  
  - Workflow overrides config file  
  - Config file overrides defaults  
- **Observability**: Execution trace, preview, logs  

## Typical Workflow Steps

1. **Input Loader**: Read images from a directory  
2. **Image Preprocessing**: Contrast, normalize, resize  
3. **OCR Analysis**: Extract text for each page  
4. **Duration Calculation**: Determine how long each page stays on screen  
5. **Video Encoding**: Combine pages + durations + audio  
6. **Transitions / Audio** (optional): Add effects, background music  
7. **Output Writer**: Save final video  

---

## Workflow File Example

```toml
[node.input_loader]
path = "manga/chapter1"
recursive = true
sort_order = "natural"
extensions = ["jpg", "png"]

[node.image_preprocess]
auto_contrast = true
normalize = true
resize = "1920x1080"
grayscale = false
denoise_level = 10

[node.ocr]
engine = "tesseract"
language = "jpn"
min_confidence = 0.75
scale_before_ocr = true
detect_bubbles = false

[node.duration]
reading_speed_wpm = 180
min_seconds = 2.0
max_seconds = 8.0
base_time = 0.5
curve = "sqrt"

[node.encoder]
codec = "h264"
fps = 30
resolution = "1920x1080"
bitrate = 5000
crf = null

[node.transition]
type = "fade"
duration = 0.4
ease = "ease-in-out"

[node.audio]
source = "bgm/music.mp3"
volume = 1.0
sync_to_pages = true
loop = true

[node.output_writer]
output_path = "output/chapter1.mp4"
overwrite = true
create_dirs = true
