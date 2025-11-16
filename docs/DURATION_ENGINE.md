# Smart Page Duration Engine

The **Smart Page Duration Engine** automatically calculates how long each manga page should remain on screen in the video. It uses **OCR text extraction** and configurable reading-speed models to generate a smooth and natural reading experience.

---

## Features

- **OCR-based text extraction** – Extracts text from each page to estimate reading time.
- **Word/Character Count** – Supports different metrics for calculating duration.
- **Custom Reading Speeds** – Configurable words-per-minute (WPM) or characters-per-minute (CPM).
- **Dynamic Timeline Generation** – Produces a list of durations for video encoding.
- **Minimum/Maximum Constraints** – Ensures pages aren’t too fast or too slow.

---

## Parameters

| Name | Type | Description |
|---|---|---|
| `reading_speed_wpm` | int | Words per minute (default: 180) |
| `min_seconds` | float | Minimum duration per page (default: 2.0s) |
| `max_seconds` | float | Maximum duration per page (default: 10.0s) |
| `base_time` | float | Constant time added to each page (default: 0.5s) |
| `count_mode` | enum(`words`,`chars`,`mixed`) | Metric used for duration calculation |
| `curve` | enum(`linear`,`log`,`sqrt`) | Mathematical model for duration scaling |

---

## Duration Calculation Models

### 1. Words Per Minute (Default)
Calculates duration based on the number of words in the page:

\[
\text{duration} = \max(\text{min_seconds}, \min(\text{max_seconds}, \frac{\text{word_count}}{\text{reading_speed_wpm}} \times 60 + \text{base_time}))
\]

### 2. Characters Per Minute
Uses character count instead of words:

\[
\text{duration} = \max(\text{min_seconds}, \min(\text{max_seconds}, \frac{\text{char_count}}{\text{reading_speed_cpm}} \times 60 + \text{base_time}))
\]

### 3. Mixed Mode
Combines word and character counts with configurable weights:

\[
\text{duration} = \max(\text{min_seconds}, \min(\text{max_seconds}, (\text{weight_words} \times \text{word_count} + \text{weight_chars} \times \text{char_count}) / \text{reading_speed_mixed} \times 60 + \text{base_time}))
\]

---

## Outputs

```json
{
  "durations": [3.2, 5.1, 2.0, 4.5, ...],
  "total_duration": 102.3
}
```

- `durations` – List of seconds per page.
- `total_duration` – Sum of all page durations.

---

## Usage Example

1. OCR Analyzer Node → extract text from pages.
2. Text Density Calculator Node → compute word/character counts.
3. Page Duration Calculator Node → generate per-page durations.
4. Feed durations into Video Encoder Node for timed rendering.

---

> This engine ensures videos are paced naturally for the reader and can handle manga with varying amounts of text per page.
