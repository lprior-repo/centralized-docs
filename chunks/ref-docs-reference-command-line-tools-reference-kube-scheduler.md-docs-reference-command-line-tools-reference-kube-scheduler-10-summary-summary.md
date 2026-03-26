---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#10-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 119
summary: || DEPRECATED: enable block profiling, if profiling is enabled. This parameter is ignored if a config file is specified in --config. | |--disable-http2-serving| || If true, HTTP2 serving will be...
---

||
DEPRECATED: enable block profiling, if profiling is enabled. This parameter is ignored if a config file is specified in --config.
|
|--disable-http2-serving|
||
If true, HTTP2 serving will be disabled [default=false]
|
|--disabled-metrics strings|
||
This flag provides an escape hatch for misbehaving metrics. You must provide the fully qualified metric name in order to disable it. Disclaimer: disabling metrics is higher in precedence than showing hidden metrics.
|
|--emulated-version strings|
||
The versions different components emulate their capabilities (APIs, features, ...) of.