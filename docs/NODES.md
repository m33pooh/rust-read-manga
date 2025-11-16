# Node Documentation

This section describes all the nodes available in the **Rust Read Manga workflow**. Nodes are split into categories for clarity.

## Table of Contents

### Input Nodes
- [Input Loader Node](nodes/00-Input-Nodes.md) – Loads images from directories.

### Image Processing Nodes
- [Image Preprocessor Node](nodes/01-Image-Nodes.md) – Basic cleanup and normalization.
- [Image Filter Node](nodes/01-Image-Nodes.md) – Apply visual filters.
- [Upscale Node](nodes/01-Image-Nodes.md) – AI or interpolation upscaling.
- [Watermark Removal Node](nodes/01-Image-Nodes.md) – Remove watermarks from images.

### Text Nodes
- [OCR Analyzer Node](nodes/02-Text-Nodes.md) – Extract text via OCR.
- [Text Density Calculator Node](nodes/02-Text-Nodes.md) – Compute text density metrics.

### Timing Nodes
- [Page Duration Calculator Node](nodes/03-Timing-Nodes.md) – Compute page durations based on text.

### Video Nodes
- [Video Encoder Node](nodes/04-Video-Nodes.md) – Encodes final video.
- [Transition Node](nodes/04-Video-Nodes.md) – Adds page transitions.
- [Audio Node](nodes/04-Video-Nodes.md) – Adds or syncs background audio.

### Output Nodes
- [Video Export Node](nodes/05-Output-Nodes.md) – Writes video to disk.
- [Log Node](nodes/05-Output-Nodes.md) – Logs workflow events.

### Logic Nodes
- [Conditional Node](nodes/06-Logic-Nodes.md) – Routes workflow based on conditions.
- [Switch Node](nodes/06-Logic-Nodes.md) – Multi-case routing.
- [Merge Node](nodes/06-Logic-Nodes.md) – Merges multiple inputs.

### Utility Nodes
- [Timer Node](nodes/07-Utility-Nodes.md) – Adds delays.
- [Logger Node](nodes/07-Utility-Nodes.md) – Logs messages or variables.
- [Random Node](nodes/07-Utility-Nodes.md) – Generates random values.
- [Counter Node](nodes/07-Utility-Nodes.md) – Tracks incremental counts.

---

> All node Markdown files are located in the [`nodes/`](nodes/) folder.
