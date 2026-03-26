---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#38-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: --config. | |--kube-api-content-type stringDefault: \"application/vnd.kubernetes.protobuf\"| || DEPRECATED: content type of requests sent to apiserver. This parameter is ignored if a config file is...
---

--config.
|
|--kube-api-content-type stringDefault: "application/vnd.kubernetes.protobuf"|
||
DEPRECATED: content type of requests sent to apiserver. This parameter is ignored if a config file is specified in --config.
|
|--kube-api-qps floatDefault: 50|
||
DEPRECATED: QPS to use while talking with kubernetes apiserver. This parameter is ignored if a config file is specified in --config.
|
|--kubeconfig string|
||
DEPRECATED: path to kubeconfig file with authorization and master location information. This parameter is ignored if a config file is specified in --config.
|
|--