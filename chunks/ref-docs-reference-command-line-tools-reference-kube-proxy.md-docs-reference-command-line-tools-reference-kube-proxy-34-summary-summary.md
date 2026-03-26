---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#34-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 119
summary: '2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate iptables resync. | |--iptables-sync-period durationDefault: 30s| || An interval (e.g. '5s', '1m',...
---

'2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate iptables resync.
|
|--iptables-sync-period durationDefault: 30s|
||
An interval (e.g. '5s', '1m', '2h22m') indicating how frequently various re-synchronizing and cleanup operations are performed. Must be greater than 0.
|
|--ipvs-exclude-cidrs strings|
||
A comma-separated list of CIDRs which the ipvs proxier should not touch when cleaning up IPVS rules.
|
|--