---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#2-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1014
summary: |--add-dir-header| ||If true, adds the file directory to the header of the log messages| |--alsologtostderr| ||log to standard error as well as files| |--as string| ||Username to impersonate for the...
---

|--add-dir-header|
||If true, adds the file directory to the header of the log messages|
|--alsologtostderr|
||log to standard error as well as files|
|--as string|
||Username to impersonate for the operation|
|--as-group stringArray|
||Group to impersonate for the operation, this flag can be repeated to specify multiple groups.|
|--azure-container-registry-config string|
||Path to the file containing Azure container registry configuration information.|
|--cache-dir stringDefault: "$HOME/.kube/cache"|
||Default cache directory|
|--certificate-authority string|
||Path to a cert file for the certificate authority|
|--client-certificate string|
||Path to a client certificate file for TLS|
|--client-key string|
||Path to a client key file for TLS|
|--cloud-provider-gce-l7lb-src-cidrs cidrsDefault: 130.211.0.0/22,35.191.0.0/16|
||CIDRs opened in GCE firewall for L7 LB traffic proxy &amp; health checks|
|--cloud-provider-gce-lb-src-cidrs cidrsDefault: 130.211.0.0/22,209.85.152.0/22,209.85.204.0/22,35.191.0.0/16|
||CIDRs opened in GCE firewall for L4 LB traffic proxy &amp; health checks|
|--cluster string|
||The name of the kubeconfig cluster to use|
|--context string|
||The name of the kubeconfig context to use|
|--default-not-ready-toleration-seconds intDefault: 300|
||Indicates the tolerationSeconds of the toleration for notReady:NoExecute that is added by default to every pod that does not already have such a toleration.|
|--default-unreachable-toleration-seconds intDefault: 300|
||Indicates the tolerationSeconds of the toleration for unreachable:NoExecute that is added by default to every pod that does not already have such a toleration.|
|-h, --help|
||help for kubectl|
|--insecure-skip-tls-verify|
||If true, the server's certificate will not be checked for validity. This will make your HTTPS connections insecure|
|--kubeconfig string|
||Path to the kubeconfig file to use for CLI requests.|
|--log-backtrace-at traceLocationDefault: :0|
||when logging hits line file:N, emit a stack trace|
|--log-dir string|
||If non-empty, write log files in this directory|
|--log-file string|
||If non-empty, use this log file|
|--log-file-max-size uintDefault: 1800|
||Defines the maximum size a log file can grow to. Unit is megabytes. If the value is 0, the maximum file size is unlimited.|
|--log-flush-frequency durationDefault: 5s|
||Maximum number of seconds between log flushes|
|--logtostderrDefault: true|
||log to standard error instead of files|
|--match-server-version|
||Require server version to match client version|
|-n, --namespace string|
||If present, the namespace scope for this CLI request|
|--one-output|
||If true, only write logs to their native severity level (vs also writing to each lower severity level)|
|--password string|
||Password for basic authentication to the API server|
|--profile stringDefault: "none"|
||Name of profile to capture. One of (none|cpu|heap|goroutine|threadcreate|block|mutex)|
|--profile-output stringDefault: "profile.pprof"|
||Name of the file to write the profile to|
|--request-timeout stringDefault: "0"|
||The length of time to wait before giving up on a single server request. Non-zero values should contain a corresponding time unit (e.g. 1s, 2m, 3h). A value of zero means don't timeout requests.|
|-s, --server string|
||The address and port of the Kubernetes API server|
|--skip-headers|
||If true, avoid header prefixes in the log messages|
|--skip-log-headers|
||If true, avoid headers when opening log files|
|--stderrthreshold severityDefault: 2|
||logs at or above this threshold go to stderr|
|--tls-server-name string|
||Server name to use for server certificate validation. If it is not provided, the hostname used to contact the server is used|
|--token string|
||Bearer token for authentication to the API server|
|--user string|
||The name of the kubeconfig user to use|
|--username string|
||Username for basic authentication to the API server|
|-v, --v Level|
||number for the log level verbosity|
|--version version[=true]|
||Print version information and quit|
|--vmodule moduleSpec|