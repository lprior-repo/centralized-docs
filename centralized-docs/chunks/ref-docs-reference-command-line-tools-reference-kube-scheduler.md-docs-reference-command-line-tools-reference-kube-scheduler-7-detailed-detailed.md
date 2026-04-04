---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#7-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1024
summary: \"text\". | |--master string| || The address of the Kubernetes API server (overrides any value in kubeconfig) | |--min-compatibility-version strings| || The min version of control plane components the...
---

"text".
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
List of request headers to inspect for UIDs. X-Remote-Uid is suggested. Requires the RemoteRequestHeaderUID feature to be enabled.
|
|--requestheader-username-headers stringsDefault: "x-remote-user"|
||
List of request headers to inspect for usernames. X-Remote-User is common.
|
|--secure-port intDefault: 10259|
||
The port on which to serve HTTPS with authentication and authorization. If 0, don't serve HTTPS at all.
|
|--show-hidden-metrics-for-version string|
||
The previous version for which you want to show hidden metrics. Only the previous minor version is meaningful, other values will not be allowed. The format is &lt;major&gt;.&lt;minor&gt;, e.g.: '1.16'. The purpose of this format is make sure you have the opportunity to notice if the next release hides additional metrics, rather than being surprised when they are permanently removed in the release after that.
|
|--tls-cert-file string|
||
File containing the default x509 Certificate for HTTPS. (CA cert, if any, concatenated after server cert). If HTTPS serving is enabled, and --tls-cert-file and --tls-private-key-file are not provided, a self-signed certificate and key are generated for the public address and saved to the directory specified by --cert-dir.
|
|--tls-cipher-suites strings|
||
Comma-separated list of cipher suites for the server. If omitted, the default Go cipher suites will be used.
Preferred values: TLS\_AES\_128\_GCM\_SHA256, TLS\_AES\_256\_GCM\_SHA384, TLS\_CHACHA20\_POLY1305\_SHA256, TLS\_ECDHE\_ECDSA\_WITH\_AES\_128\_CBC\_SHA, TLS\_ECDHE\_ECDSA\_WITH\_AES\_128\_GCM\_SHA256, TLS\_ECDHE\_ECDSA\_WITH\_AES\_256\_CBC\_SHA, TLS\_ECDHE\_ECDSA\_WITH\_AES\_256\_GCM\_SHA384, TLS\_ECDHE\_ECDSA\_WITH\_CHACHA20\_POLY1305, TLS\_ECDHE\_ECDSA\_WITH\_CHACHA20\_POLY1305\_SHA256, TLS\_ECDHE\_RSA\_WITH\