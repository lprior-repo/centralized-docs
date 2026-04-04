---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#52-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 124
summary: * **nodeInfo.kubeProxyVersion** (string), required Deprecated: KubeProxy Version reported by the node. * **nodeInfo.kubeletVersion** (string), required Kubelet Version reported by the node. *...
---

* **nodeInfo.kubeProxyVersion** (string), required
Deprecated: KubeProxy Version reported by the node.
* **nodeInfo.kubeletVersion** (string), required
Kubelet Version reported by the node.
* **nodeInfo.machineID** (string), required
MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: [http://man7.org/linux/man-pages/man5/machine-id.5.html](http://man7.org/linux/man-pages/man5/machine-id.5.html)