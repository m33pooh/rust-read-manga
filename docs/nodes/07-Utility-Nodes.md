# Utility Nodes

## Timer Node  
Delays the workflow for a specified duration.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `duration` | float | Delay in seconds |
| `repeat` | int? | Number of repeats (optional) |

**Outputs**  
```json
{ "completed": true, "elapsed": 5.0 }
```

---

## Logger Node  
Logs messages or variables for debugging.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `message` | string | Message to log |
| `level` | enum(`debug`,`info`,`warn`,`error`) | Log level |
| `include_timestamp` | bool | Include timestamp in log |

**Outputs**  
```json
{ "logged": true }
```

---

## Random Node  
Generates random values.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `type` | enum(`int`,`float`,`choice`) | Type of random output |
| `min` | number? | Minimum value (for int/float) |
| `max` | number? | Maximum value (for int/float) |
| `choices` | list? | List of options if type=`choice` |

**Outputs**  
```json
{ "value": 42 }
```

---

## Counter Node  
Keeps track of counts across executions.

**Parameters**  
| Name | Type | Description |
|---|---|---|
| `start` | int | Initial value |
| `step` | int | Increment per trigger |
| `reset_on_start` | bool | Reset counter when workflow starts |

**Outputs**  
```json
{ "count": 7 }
```
