---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#6-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 123
summary: | |--conntrack-max-per-core int32Default: 32768| || Maximum number of NAT connections to track per CPU core (0 to leave the limit as-is and ignore conntrack-min). | |--conntrack-min int32Default:...
---

|
|--conntrack-max-per-core int32Default: 32768|
||
Maximum number of NAT connections to track per CPU core (0 to leave the limit as-is and ignore conntrack-min).
|
|--conntrack-min int32Default: 131072|
||
Minimum number of conntrack entries to allocate, regardless of conntrack-max-per-core (set conntrack-max-per-core=0 to leave the limit as-is).
|
|--conntrack-tcp-be-liberal|
||
Enable liberal mode for tracking TCP packets by setting nf\_conntrack\_tcp\_be\_liberal to 1
|
|--