---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#44-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: | |--metrics-bind-address ipportDefault: 127.0.0.1:10249| || The IP address and port for the metrics server to serve on, defaulting to \"127.0.0.1:10249\". (Set to \"0.0.0.0:10249\" / \"[::]:10249\" to...
---

|
|--metrics-bind-address ipportDefault: 127.0.0.1:10249|
||
The IP address and port for the metrics server to serve on, defaulting to "127.0.0.1:10249". (Set to "0.0.0.0:10249" / "[::]:10249" to bind on all interfaces.) Set empty to disable. This parameter is ignored if a config file is specified by --config.
|
|--nodeport-addresses strings|
||
A list of CIDR ranges that contain valid node IPs, or alternatively, the single string 'primary'