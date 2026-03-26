---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#10-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 512
summary: kube:WatchCacheInitializationPostStartHook=true|false (BETA - default=false) kube:WatchList=true|false (BETA - default=true) kube:WatchListClient=true|false (BETA - default=true)...
---

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