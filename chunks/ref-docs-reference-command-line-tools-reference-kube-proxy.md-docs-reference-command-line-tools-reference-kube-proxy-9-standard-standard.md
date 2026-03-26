---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#9-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 500
summary: || If non-empty, will be used as the name of the Node that kube-proxy is running on. If unset, the node name is assumed to be the same as the node's hostname. | |--init-only| || If true, perform any...
---

||
If non-empty, will be used as the name of the Node that kube-proxy is running on. If unset, the node name is assumed to be the same as the node's hostname.
|
|--init-only|
||
If true, perform any initialization steps that must be done with full root privileges, and then exit. After doing this, you can run kube-proxy again with only the CAP\_NET\_ADMIN capability.
|
|--iptables-localhost-nodeportsDefault: true|
||
If false, kube-proxy will disable the legacy behavior of allowing NodePort services to be accessed via localhost. (Applies only to iptables mode and IPv4; localhost NodePorts are never allowed with other proxy modes or with IPv6.)
|
|--iptables-masquerade-bit int32Default: 14|
||
If using the iptables or ipvs proxy mode, the bit of the fwmark space to mark packets requiring SNAT with. Must be within the range [0, 31].
|
|--iptables-min-sync-period durationDefault: 1s|
||
The minimum period between iptables rule resyncs (e.g. '5s', '1m', '2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate iptables resync.
|
|--iptables-sync-period durationDefault: 30s|
||
An interval (e.g. '5s', '1m', '2h22m') indicating how frequently various re-synchronizing and cleanup operations are performed. Must be greater than 0.
|
|--ipvs-exclude-cidrs strings|
||
A comma-separated list of CIDRs which the ipvs proxier should not touch when cleaning up IPVS rules.
|
|--ipvs-min-sync-period durationDefault: 1s|
||
The minimum period between IPVS rule resyncs (e.g. '5s', '1m', '2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate IPVS resync.
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
An interval (e.g. '5s', '1m', '2h22m'