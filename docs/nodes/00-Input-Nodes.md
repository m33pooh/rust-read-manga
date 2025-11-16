# Input Nodes

## Input Loader Node  
Loads images from a directory and emits a list of pages.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `path` | string | Path to the input folder |
| `recursive` | bool | Whether to scan subfolders |
| `sort_order` | enum(`asc`,`desc`,`natural`) | Sorting strategy |
| `extensions` | list | Supported file extensions (e.g. `["jpg","png"]`) |

**Outputs**  
```json
{
  "pages": ["page1.jpg", "page2.png", ...],
  "total_pages": 128
}
```
