---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#4-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 88
summary: --config. | |--bind-address-hard-fail| || If true kube-proxy will treat failure to bind to a port as fatal and exit | |--cleanup| || If true cleanup iptables and ipvs rules and exit. |...
---

--config.
|
|--bind-address-hard-fail|
||
If true kube-proxy will treat failure to bind to a port as fatal and exit
|
|--cleanup|
||
If true cleanup iptables and ipvs rules and exit.
|
|--cluster-cidr string|
||
The CIDR range of the pods in the cluster. (For dual-stack clusters, this can be a comma-separated dual-stack pair of CIDR ranges.). When --