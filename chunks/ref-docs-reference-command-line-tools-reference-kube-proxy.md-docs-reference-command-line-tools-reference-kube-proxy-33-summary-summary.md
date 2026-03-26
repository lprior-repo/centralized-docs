---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#33-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 119
summary: | |--iptables-masquerade-bit int32Default: 14| || If using the iptables or ipvs proxy mode, the bit of the fwmark space to mark packets requiring SNAT with. Must be within the range [0, 31]. |...
---

|
|--iptables-masquerade-bit int32Default: 14|
||
If using the iptables or ipvs proxy mode, the bit of the fwmark space to mark packets requiring SNAT with. Must be within the range [0, 31].
|
|--iptables-min-sync-period durationDefault: 1s|
||
The minimum period between iptables rule resyncs (e.g. '5s', '1m', '2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate iptables resync.
|
|--