---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#35-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: || A comma-separated list of CIDRs which the ipvs proxier should not touch when cleaning up IPVS rules. | |--ipvs-min-sync-period durationDefault: 1s| || The minimum period between IPVS rule resyncs...
---

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
Enable strict ARP by setting arp\