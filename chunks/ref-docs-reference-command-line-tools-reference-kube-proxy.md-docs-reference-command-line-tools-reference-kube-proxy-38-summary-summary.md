---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#38-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 119
summary: || The timeout for IPVS UDP packets, 0 to leave as-is. (e.g. '5s', '1m', '2h22m'). | |--kube-api-burst int32Default: 10| || Burst to use while talking with kubernetes apiserver |...
---

||
The timeout for IPVS UDP packets, 0 to leave as-is. (e.g. '5s', '1m', '2h22m').
|
|--kube-api-burst int32Default: 10|
||
Burst to use while talking with kubernetes apiserver
|
|--kube-api-content-type stringDefault: "application/vnd.kubernetes.protobuf"|
||
Content type of requests sent to apiserver.
|
|--kube-api-qps floatDefault: 5|
||
QPS to use while talking with kubernetes apiserver
|
|--kubeconfig string|
||