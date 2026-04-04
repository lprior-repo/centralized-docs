---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#6-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1004
summary: kube:VolumeLimitScaling=true|false (ALPHA - default=false) kube:WatchCacheInitializationPostStartHook=true|false (BETA - default=false) kube:WatchList=true|false (BETA - default=true)...
---

kube:VolumeLimitScaling=true|false (ALPHA - default=false)
kube:WatchCacheInitializationPostStartHook=true|false (BETA - default=false)
kube:WatchList=true|false (BETA - default=true)
kube:WatchListClient=true|false (BETA - default=true)
kube:WindowsCPUAndMemoryAffinity=true|false (ALPHA - default=false)
kube:WindowsGracefulNodeShutdown=true|false (BETA - default=true)
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
DEPRECATED: content type of requests sent to apiserver. This parameter is ignored if a config file is specified in --config.
|
|--kube-api-qps floatDefault: 50|
||
DEPRECATED: QPS to use while talking with kubernetes apiserver. This parameter is ignored if a config file is specified in --config.
|
|--kubeconfig string|
||
DEPRECATED: path to kubeconfig file with authorization and master location information. This parameter is ignored if a config file is specified in --config.
|
|--leader-electDefault: true|
||
Start a leader election client and gain leadership before executing the main loop. Enable this when running replicated components for high availability.
|
|--leader-elect-lease-duration durationDefault: 15s|
||
The duration that non-leader candidates will wait after observing a leadership renewal until attempting to acquire leadership of a led but unrenewed leader slot. This is effectively the maximum duration that a leader can be stopped before it is replaced by another candidate. This is only applicable if leader election is enabled.
|
|--leader-elect-renew-deadline durationDefault: 10s|
||
The interval between attempts by the acting master to renew a leadership slot before it stops leading. This must be less than the lease duration. This is only applicable if leader election is enabled.
|
|--leader-elect-resource-lock stringDefault: "leases"|
||
The type of resource object that is used for locking during leader election. Supported options are 'leases'.
|
|--leader-elect-resource-name stringDefault: "kube-scheduler"|
||
The name of resource object that is used for locking during leader election.
|
|--leader-elect-resource-namespace stringDefault: "kube-system"|
||
The namespace of resource object that is used for locking during leader election.
|
|--leader-elect-retry-period durationDefault: 2s|
||
The duration the clients should wait between attempting acquisition and renewal of a leadership. This is only applicable if leader election is enabled.
|
|--log-flush-frequency durationDefault: 5s|
||
Maximum number of seconds between log flushes
|
|--log-text-info-buffer-size quantity|
||
[Alpha] In text format with split output streams, the info messages can be buffered for a while to increase performance. The default value of zero bytes disables buffering. The size can be specified as number of bytes (512), multiples of 1000 (1K), multiples of 1024 (2Ki), or powers of those (3M, 4G, 5Mi, 6Gi). Enable the LoggingAlphaOptions feature gate to use this.
|
|--log-text-split-stream|
||
[Alpha] In text format, write error messages to stderr and info messages to stdout. The default is to write a single stream to stdout. Enable the LoggingAlphaOptions feature gate to use this.
|
|--logging-format stringDefault: "text"|
||
Sets the log format. Permitted formats: "text".
|
|--master string|
||
The address of the Kubernetes API server (overrides any value in kubeconfig)
|
|--min-compatibility-version strings|
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
If true, SO\