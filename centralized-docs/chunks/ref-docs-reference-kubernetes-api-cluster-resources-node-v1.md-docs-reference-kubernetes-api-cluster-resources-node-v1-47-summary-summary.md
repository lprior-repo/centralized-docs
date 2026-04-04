---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#47-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 91
summary: * **daemonEndpoints.kubeletEndpoint** (DaemonEndpoint) Endpoint on which Kubelet is listening. *DaemonEndpoint contains information about a single Daemon endpoint.* *...
---

* **daemonEndpoints.kubeletEndpoint** (DaemonEndpoint)
Endpoint on which Kubelet is listening.
*DaemonEndpoint contains information about a single Daemon endpoint.*
* **daemonEndpoints.kubeletEndpoint.Port** (int32), required
Port number of the given endpoint.
* **declaredFeatures** ([]string)
*Atomic: will be replaced during a merge*
DeclaredFeatures represents the features related to feature gates that are declared by the node.