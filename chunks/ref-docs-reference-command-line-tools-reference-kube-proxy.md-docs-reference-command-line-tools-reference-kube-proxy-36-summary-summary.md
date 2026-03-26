---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#36-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: || The ipvs scheduler type when proxy mode is ipvs | |--ipvs-strict-arp| || Enable strict ARP by setting arp\_ignore to 1 and arp\_announce to 2 | |--ipvs-sync-period durationDefault: 30s| || An...
---

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
The timeout for idle IPVS TCP connections, 0 to leave as-is. (e.g.