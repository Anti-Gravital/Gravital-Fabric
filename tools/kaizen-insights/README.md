# kaizen-insights

Offline aggregator for sanitised telemetry. It groups error and crash events
by fingerprint, computes p95 latency for performance events and counts usage.
It writes `latest.json` and a dated JSON file to
`docs/kaizen/data/insights/`.

Run it through `scripts/kaizen-insights`, which builds the release binary on
demand. Environment variables: `KZ_TELEMETRY_GLOB` (directory of `.jsonl`
files), `KZ_INSIGHT_MIN_COUNT`, `KZ_INSIGHT_MIN_USERS`, `KZ_PERF_P95_MS`.
`--create-issues` proposes deduplicated issues with `gh`.

The glob is simplified to a directory scan of `.jsonl` files. The tool uses
only `serde` and `serde_json`; redaction uses the standard library.
