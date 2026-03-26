---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#11-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 512
summary: leader-elect-renew-deadline durationDefault: 10s| || The interval between attempts by the acting master to renew a leadership slot before it stops leading. This must be less than the lease duration....
---

leader-elect-renew-deadline durationDefault: 10s|
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
If true, SO