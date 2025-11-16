# Logic Nodes

## Conditional Node  
Routes workflow based on a condition.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `condition_type` | enum(`eq`,`neq`,`gt`,`lt`,`gte`,`lte`,`contains`) | Comparison type |
| `value` | string/number | Value to compare against |
| `target_input` | string | Name of input to evaluate |

**Outputs**  
- `true` branch  
- `false` branch  

```json
{
  "matched": true,
  "branch": "true"
}
```

---

## Switch Node  
Routes to multiple outputs based on case matching.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `cases` | list | Array of values to match |
| `default` | string? | Default branch if no match |

**Outputs**  
- One output per case  
- Optional default output  

```json
{ "case_matched": "case_2" }
```

---

## Merge Node  
Merges multiple inputs into a single output.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `mode` | enum(`concat`,`first`,`last`,`sum`) | Merge strategy |

**Outputs**  
```json
{ "merged_data": [...] }
```
