---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#3-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 510
summary: |-h, --help| ||help for kubectl| |--insecure-skip-tls-verify| ||If true, the server's certificate will not be checked for validity. This will make your HTTPS connections insecure| |--kubeconfig...
---

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