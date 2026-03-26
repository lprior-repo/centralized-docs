---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#46-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 112
summary: --config. | |--one\_output| || If true, only write logs to their native severity level (vs also writing to each lower severity level; no effect when -logtostderr=true) | |--oom-score-adj...
---

--config.
|
|--one\_output|
||
If true, only write logs to their native severity level (vs also writing to each lower severity level; no effect when -logtostderr=true)
|
|--oom-score-adj int32Default: -999|
||
The oom-score-adj value for kube-proxy process. Values must be within the range [-1000, 1000]. This parameter is ignored if a config file is specified by --config.
|
|--pod-bridge-interface string|
||
A bridge interface name. When --