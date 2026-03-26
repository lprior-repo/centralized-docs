---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#47-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: --config. | |--pod-bridge-interface string| || A bridge interface name. When --detect-local-mode is set to BridgeInterface, kube-proxy will consider traffic to be local if it originates from this...
---

--config.
|
|--pod-bridge-interface string|
||
A bridge interface name. When --detect-local-mode is set to BridgeInterface, kube-proxy will consider traffic to be local if it originates from this bridge.
|
|--pod-interface-name-prefix string|
||
An interface name prefix. When --detect-local-mode is set to InterfaceNamePrefix, kube-proxy will consider traffic to be local if it originates from any interface whose name begins with this prefix.
|
|--profiling|
||
If true enables profiling via web interface on /debug/pprof handler. This parameter is ignored if a config file is specified by --config.
|
|--