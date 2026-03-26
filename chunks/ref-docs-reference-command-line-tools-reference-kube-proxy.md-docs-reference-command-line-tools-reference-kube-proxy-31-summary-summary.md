---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#31-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 109
summary: || The IP address and port for the health check server to serve on, defaulting to \"0.0.0.0:10256\". This parameter is ignored if a config file is specified by --config. | |-h, --help| || help for...
---

||
The IP address and port for the health check server to serve on, defaulting to "0.0.0.0:10256". This parameter is ignored if a config file is specified by --config.
|
|-h, --help|
||
help for kube-proxy
|
|--hostname-override string|
||
If non-empty, will be used as the name of the Node that kube-proxy is running on. If unset, the node name is assumed to be the same as the node's hostname.
|
|--init-only|
||