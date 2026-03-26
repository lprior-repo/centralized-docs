---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#6-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1010
summary: Enable strict ARP by setting arp\_ignore to 1 and arp\_announce to 2 | |--ipvs-sync-period durationDefault: 30s| || An interval (e.g. '5s', '1m', '2h22m') indicating how frequently various...
---

Enable strict ARP by setting arp\_ignore to 1 and arp\_announce to 2
|
|--ipvs-sync-period durationDefault: 30s|
||
An interval (e.g. '5s', '1m', '2h22m') indicating how frequently various re-synchronizing and cleanup operations are performed. Must be greater than 0.
|
|--ipvs-tcp-timeout duration|
||
The timeout for idle IPVS TCP connections, 0 to leave as-is. (e.g. '5s', '1m', '2h22m').
|
|--ipvs-tcpfin-timeout duration|
||
The timeout for IPVS TCP connections after receiving a FIN packet, 0 to leave as-is. (e.g. '5s', '1m', '2h22m').
|
|--ipvs-udp-timeout duration|
||
The timeout for IPVS UDP packets, 0 to leave as-is. (e.g. '5s', '1m', '2h22m').
|
|--kube-api-burst int32Default: 10|
||
Burst to use while talking with kubernetes apiserver
|
|--kube-api-content-type stringDefault: "application/vnd.kubernetes.protobuf"|
||
Content type of requests sent to apiserver.
|
|--kube-api-qps floatDefault: 5|
||
QPS to use while talking with kubernetes apiserver
|
|--kubeconfig string|
||
Path to kubeconfig file with authorization information (the master location can be overridden by the master flag).
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
|--log\_backtrace\_at &lt;a string in the form 'file:N'&gt;Default: :0|
||
when logging hits line file:N, emit a stack trace
|
|--log\_dir string|
||
If non-empty, write log files in this directory (no effect when -logtostderr=true)
|
|--log\_file string|
||
If non-empty, use this log file (no effect when -logtostderr=true)
|
|--log\_file\_max\_size uintDefault: 1800|
||
Defines the maximum size a log file can grow to (no effect when -logtostderr=true). Unit is megabytes. If the value is 0, the maximum file size is unlimited.
|
|--logging-format stringDefault: "text"|
||
Sets the log format. Permitted formats: "text".
|
|--logtostderrDefault: true|
||
log to standard error instead of files
|
|--masquerade-all|
||
SNAT all traffic sent via Service cluster IPs. This may be required with some CNI plugins. Only supported on Linux.
|
|--master string|
||
The address of the Kubernetes API server (overrides any value in kubeconfig)
|
|--metrics-bind-address ipportDefault: 127.0.0.1:10249|
||
The IP address and port for the metrics server to serve on, defaulting to "127.0.0.1:10249". (Set to "0.0.0.0:10249" / "[::]:10249" to bind on all interfaces.) Set empty to disable. This parameter is ignored if a config file is specified by --config.
|
|--nodeport-addresses strings|
||
A list of CIDR ranges that contain valid node IPs, or alternatively, the single string 'primary'. If set to a list of CIDRs, connections to NodePort services will only be accepted on node IPs in one of the indicated ranges. If set to 'primary', NodePort services will only be accepted on the node's primary IP(s) according to the Node object. If unset, NodePort connections will be accepted on all local IPs. This parameter is ignored if a config file is specified by --config.
|
|--one\_output|
||
If true, only write logs to their native severity level (vs also writing to each lower severity level; no effect when -logtostderr=true)
|
|--oom-score-adj int32Default: -999|
||