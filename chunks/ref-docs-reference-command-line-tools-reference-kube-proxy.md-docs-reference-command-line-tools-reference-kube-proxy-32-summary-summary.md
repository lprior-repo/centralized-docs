---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#32-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 125
summary: 's hostname. | |--init-only| || If true, perform any initialization steps that must be done with full root privileges, and then exit. After doing this, you can run kube-proxy again with only the...
---

's hostname.
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