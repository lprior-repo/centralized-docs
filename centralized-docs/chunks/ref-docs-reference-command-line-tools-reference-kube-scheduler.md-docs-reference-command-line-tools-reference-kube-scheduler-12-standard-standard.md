---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#12-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 510
summary: || The min version of control plane components the server should be compatible with. Must be less or equal to the emulated-version. Version format could only be major.minor, for example:...
---

||
The min version of control plane components the server should be compatible with.
Must be less or equal to the emulated-version. Version format could only be major.minor, for example: '--min-compatibility-version=wardle=1.2,kube=1.31'.
Options are: kube=1.32..1.35(default:1.34)
If the component is not specified, defaults to "kube"
|
|--permit-address-sharing|
||
If true, SO\_REUSEADDR will be used when binding the port. This allows binding to wildcard IPs like 0.0.0.0 and specific IPs in parallel, and it avoids waiting for the kernel to release sockets in TIME\_WAIT state. [default=false]
|
|--permit-port-sharing|
||
If true, SO\_REUSEPORT will be used when binding the port, which allows more than one instance to bind on the same address and port. [default=false]
|
|--pod-max-in-unschedulable-pods-duration durationDefault: 5m0s|
||
DEPRECATED: the maximum time a pod can stay in unschedulablePods. If a pod stays in unschedulablePods for longer than this value, the pod will be moved from unschedulablePods to backoffQ or activeQ. This flag is deprecated and will be removed in a future version.
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
Root certificate bundle to use to verify client certificates on incoming requests before trusting usernames in headers specified by --requestheader-username-headers. WARNING: generally do not depend on authorization being already done for incoming requests.
|
|--requestheader-extra-headers-prefix stringsDefault: "x-remote-extra-"|
||
List of request header prefixes to inspect. X-Remote-Extra- is suggested.
|
|--requestheader-group-headers stringsDefault: "x-remote-group"|
||
List of request headers to inspect for groups. X-Remote-Group is suggested.
|
|--requestheader-uid-headers strings|
||