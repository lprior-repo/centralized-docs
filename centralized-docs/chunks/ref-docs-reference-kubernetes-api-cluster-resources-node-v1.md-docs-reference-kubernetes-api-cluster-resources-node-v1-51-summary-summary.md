---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#51-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 125
summary: * **nodeInfo.bootID** (string), required Boot ID reported by the node. * **nodeInfo.containerRuntimeVersion** (string), required ContainerRuntime Version reported by the node through runtime remote...
---

* **nodeInfo.bootID** (string), required
Boot ID reported by the node.
* **nodeInfo.containerRuntimeVersion** (string), required
ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).
* **nodeInfo.kernelVersion** (string), required
Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
* **nodeInfo.kubeProxyVersion** (string), required
Deprecated: KubeProxy Version reported by the node.