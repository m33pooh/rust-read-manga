# Output Nodes

## Video Export Node  
Final output node that writes the video to disk.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `path` | string | Output file path |
| `format` | enum(`mp4`,`avi`,`mkv`) | Container format |
| `overwrite` | bool | Overwrite if file exists |
| `include_metadata` | bool | Embed metadata such as title/author |

**Inputs**  
- video stream from Video Encoder Node  
- optional audio  

**Outputs**  
```json
{ "success": true, "output_file": "video/output.mp4" }
```

---

## Log Node  
Logs workflow events for monitoring or debugging.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `level` | enum(`debug`,`info`,`warn`,`error`) | Log level |
| `file` | string? | Optional file path for log output |
| `console` | bool | Print logs to console |
| `format` | enum(`text`,`json`) | Log format |

**Outputs**  
```json
{ "logs_written": 120 }
```
