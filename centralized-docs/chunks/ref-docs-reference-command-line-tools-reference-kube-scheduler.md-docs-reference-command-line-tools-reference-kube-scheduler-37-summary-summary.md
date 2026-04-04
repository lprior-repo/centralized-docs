---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#37-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 117
summary: | |-h, --help| || help for kube-scheduler | |--http2-max-streams-per-connection int| || The limit that the server gives to clients for the maximum number of streams in an HTTP/2 connection. Zero...
---

|
|-h, --help|
||
help for kube-scheduler
|
|--http2-max-streams-per-connection int|
||
The limit that the server gives to clients for the maximum number of streams in an HTTP/2 connection. Zero means to use golang's default.
|
|--kube-api-burst int32Default: 100|
||
DEPRECATED: burst to use while talking with kubernetes apiserver. This parameter is ignored if a config file is specified in --config.
|
|--kube-api-content-type stringDefault: "application/vnd.kubernetes.protobuf"|
||