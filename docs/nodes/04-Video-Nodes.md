# Video Nodes

## Video Encoder Node  
Encodes the final video from images + durations.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `codec` | enum(`h264`,`h265`,`vp9`,`theora`) | Codec choice |
| `fps` | int | Frame rate |
| `resolution` | string | e.g. `"1920x1080"` |
| `bitrate` | int | Bitrate in kbps |
| `audio` | string? | Path to audio file |
| `crf` | int? | Quality (if using CRF mode) |

**Inputs**  
- processed pages  
- durations  

**Outputs**  
```json
{ "video_path": "output/video.mp4" }
```

---

## Transition Node  
Applies page transitions.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `type` | enum(`fade`,`zoom`,`slide`,`none`) | Transition style |
| `duration` | float | How long transition lasts |
| `ease` | enum(`linear`,`ease-in`,`ease-out`,`ease-in-out`) | Easing curve |

---

## Audio Node  
Adds or configures background audio.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `source` | string | Audio file path |
| `volume` | float | Playback volume (0.0–2.0) |
| `sync_to_pages` | bool | Sync audio with page transitions |
| `loop` | bool | Loop audio if shorter than video |
