---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#16-summary
chunk_level: summary
chunk_type: table
heading: Environment variables
token_count: 122
summary: |KUBECTL\_REMOTE\_COMMAND\_WEBSOCKETS| ||When set to true, the kubectl exec, cp, and attach commands will attempt to stream using the websockets protocol. If the upgrade to websockets fails, the...
---

|KUBECTL\_REMOTE\_COMMAND\_WEBSOCKETS|
||When set to true, the kubectl exec, cp, and attach commands will attempt to stream using the websockets protocol. If the upgrade to websockets fails, the commands will fallback to use the current SPDY protocol.|
|KUBECTL\_KUBERC|
||When set to true, kuberc file is taken into account to define user specific preferences.|
|KUBECTL\_KYAML|
||When set to true, kubectl is capable of producing Kubernetes-specific dialect of YAML output format.|