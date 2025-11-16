# Text Analysis Nodes

## OCR Analyzer Node  
Extracts text from manga pages.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `engine` | enum(`tesseract`,`paddleocr`) | OCR backend |
| `language` | string | Language code (e.g. `jpn`) |
| `min_confidence` | float | Minimum acceptable confidence |
| `scale_before_ocr` | bool | Upscale pages before OCR |
| `detect_bubbles` | bool | Try to detect speech bubbles |

**Outputs**  
```json
{
  "ocr_results": [
    { "page": 1, "text": "...", "confidence": 0.9, "word_count": 12, "char_count": 45 },
    ...
  ]
}
```

---

## Text Density Calculator Node  
Generates density metrics from OCR.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `mode` | enum(`words`,`chars`,`mixed`) | Metric to use |
| `weight_chars` | float | Weight for char count (if mixed) |
| `weight_words` | float | Weight for word count (if mixed) |

**Outputs**  
```json
{
  "density_scores": [0.3, 0.55, 0.1, ...]
}
```
