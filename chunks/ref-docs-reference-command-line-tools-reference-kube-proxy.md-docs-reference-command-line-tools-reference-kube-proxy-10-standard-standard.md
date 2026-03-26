---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#10-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 487
summary: | |--ipvs-scheduler string| || The ipvs scheduler type when proxy mode is ipvs | |--ipvs-strict-arp| || Enable strict ARP by setting arp\_ignore to 1 and arp\_announce to 2 | |--ipvs-sync-period...
---

|
|--ipvs-scheduler string|
||
The ipvs scheduler type when proxy mode is ipvs
|
|--ipvs-strict-arp|
||
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
[Alpha]