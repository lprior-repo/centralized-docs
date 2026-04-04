---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#47-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 122
summary: | |--profilingDefault: true| || DEPRECATED: enable profiling via web interface host:port/debug/pprof/. This parameter is ignored if a config file is specified in --config. |...
---

|
|--profilingDefault: true|
||
DEPRECATED: enable profiling via web interface host:port/debug/pprof/. This parameter is ignored if a config file is specified in --config.
|
|--requestheader-allowed-names strings|
||
List of client certificate common names to allow to provide usernames in headers specified by --requestheader-username-headers. If empty, any client certificate validated by the authorities in --requestheader-client-ca-file is allowed.
|
|--requestheader-client-ca-file string|
||
Root certificate bundle to use to verify client certificates on incoming requests before trusting usernames in headers specified by --