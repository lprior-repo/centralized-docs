---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#5-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 110
summary: --detect-local-mode is set to ClusterCIDR, kube-proxy will consider traffic to be local if its source IP is in this range. (Otherwise it is not used.) This parameter is ignored if a config file is...
---

--detect-local-mode is set to ClusterCIDR, kube-proxy will consider traffic to be local if its source IP is in this range. (Otherwise it is not used.) This parameter is ignored if a config file is specified by --config.
|
|--config string|
||
The path to the configuration file.
|
|--config-sync-period durationDefault: 15m0s|
||
How often configuration from the apiserver is refreshed. Must be greater than 0.
|
|--conntrack-max-per-core int32Default: 32768|
||