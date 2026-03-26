---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#3-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: |--add\_dir\_header| || If true, adds the file directory to the header of the log messages | |--alsologtostderr| || log to standard error as well as files (no effect when -logtostderr=true) |...
---

|--add\_dir\_header|
||
If true, adds the file directory to the header of the log messages
|
|--alsologtostderr|
||
log to standard error as well as files (no effect when -logtostderr=true)
|
|--bind-address stringDefault: 0.0.0.0|
||
Overrides kube-proxy's idea of what its node's primary IP is. Note that the name is a historical artifact, and kube-proxy does not actually bind any sockets to this IP. This parameter is ignored if a config file is specified by --config.
|
|--bind-address-hard-fail|