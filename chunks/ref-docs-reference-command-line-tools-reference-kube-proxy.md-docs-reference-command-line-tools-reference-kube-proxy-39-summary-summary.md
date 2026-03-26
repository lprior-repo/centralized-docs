---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#39-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 86
summary: kube-api-qps floatDefault: 5| || QPS to use while talking with kubernetes apiserver | |--kubeconfig string| || Path to kubeconfig file with authorization information (the master location can be...
---

kube-api-qps floatDefault: 5|
||
QPS to use while talking with kubernetes apiserver
|
|--kubeconfig string|
||
Path to kubeconfig file with authorization information (the master location can be overridden by the master flag).
|
|--log-flush-frequency durationDefault: 5s|
||
Maximum number of seconds between log flushes
|
|--log-text-info-buffer-size quantity|
||
[Alpha]