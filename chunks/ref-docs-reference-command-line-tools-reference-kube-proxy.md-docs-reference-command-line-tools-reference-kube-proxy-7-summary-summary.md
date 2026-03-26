---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#7-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 123
summary: || Enable liberal mode for tracking TCP packets by setting nf\_conntrack\_tcp\_be\_liberal to 1 | |--conntrack-tcp-timeout-close-wait durationDefault: 1h0m0s| || NAT timeout for TCP connections in...
---

||
Enable liberal mode for tracking TCP packets by setting nf\_conntrack\_tcp\_be\_liberal to 1
|
|--conntrack-tcp-timeout-close-wait durationDefault: 1h0m0s|
||
NAT timeout for TCP connections in the CLOSE\_WAIT state
|
|--conntrack-tcp-timeout-established durationDefault: 24h0m0s|
||
Idle timeout for established TCP connections (0 to leave as-is)
|
|--conntrack-udp-timeout duration|
||
Idle timeout for UNREPLIED UDP connections (0 to leave as-is)
|
|--