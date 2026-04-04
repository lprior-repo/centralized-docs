---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#15-standard
chunk_level: standard
chunk_type: prose
heading: NodeList
token_count: 257
summary: ## NodeList NodeList is the whole list of all Nodes which have been registered with master. * **apiVersion**: v1 * **kind**: NodeList * **metadata**...
---

## NodeList
NodeList is the whole list of all Nodes which have been registered with master.
* **apiVersion**: v1
* **kind**: NodeList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **items** ([][Node](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node)), required
List of nodes
#### Parameters
* **name** (*in path*): string, required
name of the Node
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **name** (*in path*): string, required
name of the Node
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)