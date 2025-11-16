# Image Processing Nodes

## Image Preprocessor Node  
Performs basic image cleanup and normalization.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `auto_contrast` | bool | Automatically adjust contrast |
| `normalize` | bool | Normalize color levels |
| `resize` | string? | Target size, e.g. `"1920x1080"` |
| `grayscale` | bool | Convert to grayscale |
| `denoise_level` | int (0–100) | Strength of noise reduction |

**Outputs**  
```json
{
  "processed_pages": ["processed1.jpg", "processed2.jpg", ...],
  "stats": {
    "avg_processing_time_ms": 12
  }
}
```

---

## Image Filter Node  
Apply a visual filter to each page.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `filter` | enum(`none`,`sharp`,`smooth`,`comic`,`bw`) | Filter type |
| `strength` | float | Intensity of the filter |

---

## Upscale Node  
Upscales images using AI or interpolation.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `method` | enum(`esrgan`,`waifu2x`,`bicubic`) | Algorithm to use |
| `scale` | float | Scaling multiplier (e.g. 2.0) |
| `noise_reduction` | bool | Denoise before upscaling |

---

## Watermark Removal Node  
Attempts to remove watermarks.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `method` | enum(`auto`,`mask`,`ai`) | Detection & removal method |
| `mask_path` | string? | Optional mask file path |
| `strength` | float | Inpainting strength |
