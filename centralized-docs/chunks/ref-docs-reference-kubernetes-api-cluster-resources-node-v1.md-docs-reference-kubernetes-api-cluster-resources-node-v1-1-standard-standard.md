---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#1-standard
chunk_level: standard
chunk_type: prose
heading: Node
token_count: 344
summary: # Node Node is a worker node in Kubernetes. `apiVersion: v1` `import \"k8s.io/api/core/v1\"` ## Node Node is a worker node in Kubernetes. Each node will have a unique identifier in the cache (i.e. in...
---

# Node
Node is a worker node in Kubernetes.
`apiVersion: v1`
`import "k8s.io/api/core/v1"`
## Node
Node is a worker node in Kubernetes. Each node will have a unique identifier in the cache (i.e. in etcd).
* **apiVersion**: v1
* **kind**: Node
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **spec** ([NodeSpec](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#NodeSpec))
Spec defines the behavior of a node. [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status)
* **status** ([NodeStatus](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#NodeStatus))
Most recently observed status of the node. Populated by the system. Read-only. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status)