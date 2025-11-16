# Timing Nodes

## Page Duration Calculator Node  
Calculates duration for each page based on text metrics.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `reading_speed_wpm` | int | Words per minute (e.g. 180) |
| `min_seconds` | float | Minimum duration per page |
| `max_seconds` | float | Maximum duration per page |
| `base_time` | float | Constant time added to each page |
| `curve` | enum(`linear`,`log`,`sqrt`) | Mathematical model for timing |

**Outputs**  
```json
{
  "durations": [3.2, 5.1, 2.0, ...]
}
```
