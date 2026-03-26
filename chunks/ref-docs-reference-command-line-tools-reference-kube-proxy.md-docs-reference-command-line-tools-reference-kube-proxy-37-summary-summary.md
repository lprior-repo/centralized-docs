---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#37-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: |--ipvs-tcp-timeout duration| || The timeout for idle IPVS TCP connections, 0 to leave as-is. (e.g. '5s', '1m', '2h22m'). | |--ipvs-tcpfin-timeout duration| || The timeout for IPVS TCP connections...
---

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
The timeout for IPVS UDP packets, 0 to leave as-is. (e.g. '5s',