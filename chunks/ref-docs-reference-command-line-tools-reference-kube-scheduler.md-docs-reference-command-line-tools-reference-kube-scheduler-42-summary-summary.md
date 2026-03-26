---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#42-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 125
summary: || Maximum number of seconds between log flushes | |--log-text-info-buffer-size quantity| || [Alpha] In text format with split output streams, the info messages can be buffered for a while to...
---

||
Maximum number of seconds between log flushes
|
|--log-text-info-buffer-size quantity|
||
[Alpha] In text format with split output streams, the info messages can be buffered for a while to increase performance. The default value of zero bytes disables buffering. The size can be specified as number of bytes (512), multiples of 1000 (1K), multiples of 1024 (2Ki), or powers of those (3M, 4G, 5Mi, 6Gi). Enable the LoggingAlphaOptions feature gate to use this.
|
|--log-text-split-stream|
||
[Alpha]