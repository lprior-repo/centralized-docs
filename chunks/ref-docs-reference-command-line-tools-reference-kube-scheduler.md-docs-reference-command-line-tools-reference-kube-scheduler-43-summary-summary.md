---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#43-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 116
summary: | |--log-text-split-stream| || [Alpha] In text format, write error messages to stderr and info messages to stdout. The default is to write a single stream to stdout. Enable the LoggingAlphaOptions...
---

|
|--log-text-split-stream|
||
[Alpha] In text format, write error messages to stderr and info messages to stdout. The default is to write a single stream to stdout. Enable the LoggingAlphaOptions feature gate to use this.
|
|--logging-format stringDefault: "text"|
||
Sets the log format. Permitted formats: "text".
|
|--master string|
||
The address of the Kubernetes API server (overrides any value in kubeconfig)
|
|--min-compatibility-version strings|
||
The min version of control plane components the server should be compatible with.