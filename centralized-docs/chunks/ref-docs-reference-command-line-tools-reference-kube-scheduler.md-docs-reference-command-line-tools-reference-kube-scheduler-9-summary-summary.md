---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#9-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: | |--cert-dir string| || The directory where the TLS certs are located. If --tls-cert-file and --tls-private-key-file are provided, this flag will be ignored. | |--client-ca-file string| || If set,...
---

|
|--cert-dir string|
||
The directory where the TLS certs are located. If --tls-cert-file and --tls-private-key-file are provided, this flag will be ignored.
|
|--client-ca-file string|
||
If set, any request presenting a client certificate signed by one of the authorities in the client-ca-file is authenticated with an identity corresponding to the CommonName of the client certificate.
|
|--config string|
||
The path to the configuration file.
|
|--contention-profilingDefault: true|
||
DEPRECATED: enable block profiling, if profiling is enabled. This parameter is ignored if a config file is specified in --