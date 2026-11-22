# Event schema / Esquema de eventos

JSON Lines, one event per line. Every field is sanitised at the source; see
`PRIVACY.md`. JSON Lines, un evento por linea. Todos los campos se sanean en el
origen; ver `PRIVACY.md`.

| Field / Campo | Meaning / Significado |
| --- | --- |
| `ts` | UTC ISO 8601 timestamp / marca de tiempo UTC ISO 8601 |
| `session` | Random rotating identifier, never a stable user or device ID / identificador aleatorio y rotativo |
| `version` | Application version / version de la aplicacion |
| `env` | `prod` \| `staging` \| `dev` |
| `type` | `error` \| `crash` \| `perf` \| `usage` |
| `name` | Stable event name / nombre estable del evento |
| `fingerprint` | Stable error hash (type plus stack frames without local paths) / hash estable del error |
| `message` | Sanitised text, at most 200 characters / texto saneado, maximo 200 caracteres |
| `stack_top` | Up to 5 frames, no absolute paths or user data / hasta 5 frames, sin rutas absolutas ni datos de usuario |
| `duration_ms` | Only for `type=perf` / solo para `type=perf` |
| `attrs` | Object with allowlisted keys / objeto con claves de lista permitida |

Example / Ejemplo:

```json
{"ts":"2026-11-21T10:00:00Z","session":"a91f","version":"1.4.2","env":"prod","type":"error","name":"parse.config","fingerprint":"9c1e44ab","message":"unexpected token at line 3","stack_top":["parse_config:88","load:41"],"attrs":{"os":"linux"}}
```
