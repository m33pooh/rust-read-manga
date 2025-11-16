# Node Documentation

This section describes all the nodes available in the **Rust Read Manga workflow**.

## Table of Contents

### Input Nodes
- [Input Loader](nodes/00-Input-Nodes.md) – Loads images from a directory.

### Image Processing Nodes
- [Image Preprocessor](nodes/01-Image-Nodes.md) – Performs basic image cleanup and normalization.

### Text Nodes
- [OCR Analyzer](nodes/02-Text-Nodes.md) – Extracts text from images using OCR.

### Timing Nodes
- [Page Duration Calculator](nodes/03-Timing-Nodes.md) – Computes page display durations.

### Video Nodes
- [Video Encoder](nodes/04-Video-Nodes.md) – Encodes the final video file.
- [Transition](nodes/04-Video-Nodes.md) – Adds transitions between pages.
- [Audio](nodes/04-Video-Nodes.md) – Adds a background audio track.

### Output Nodes
- [Video Exporter](nodes/05-Output-Nodes.md) – Writes the final video to disk.

---

> All node Markdown files are located in the [`nodes/`](nodes/) folder.
