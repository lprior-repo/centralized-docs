---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#5-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 95
summary: cluster string| ||The name of the kubeconfig cluster to use| |--context string| ||The name of the kubeconfig context to use| |--default-not-ready-toleration-seconds intDefault: 300| ||Indicates the...
---

cluster string|
||The name of the kubeconfig cluster to use|
|--context string|
||The name of the kubeconfig context to use|
|--default-not-ready-toleration-seconds intDefault: 300|
||Indicates the tolerationSeconds of the toleration for notReady:NoExecute that is added by default to every pod that does not already have such a toleration.|
|--default-unreachable-toleration-seconds intDefault: 300|