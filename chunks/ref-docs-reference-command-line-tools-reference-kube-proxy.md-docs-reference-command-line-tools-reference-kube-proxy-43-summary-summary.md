---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#43-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 120
summary: | |--logging-format stringDefault: \"text\"| || Sets the log format. Permitted formats: \"text\". | |--logtostderrDefault: true| || log to standard error instead of files | |--masquerade-all| || SNAT all...
---

|
|--logging-format stringDefault: "text"|
||
Sets the log format. Permitted formats: "text".
|
|--logtostderrDefault: true|
||
log to standard error instead of files
|
|--masquerade-all|
||
SNAT all traffic sent via Service cluster IPs. This may be required with some CNI plugins. Only supported on Linux.
|
|--master string|
||
The address of the Kubernetes API server (overrides any value in kubeconfig)
|
|--metrics-bind-address ipportDefault: 127.0.0.1:10249|
||