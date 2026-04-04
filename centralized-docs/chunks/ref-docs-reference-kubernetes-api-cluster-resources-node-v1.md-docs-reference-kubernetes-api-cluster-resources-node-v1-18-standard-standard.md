---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#18-standard
chunk_level: standard
chunk_type: prose
heading: NodeList
token_count: 507
summary: #### Response 200 ([Node](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node)): OK 201...
---

#### Response
200 ([Node](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node)): OK
201 ([Node](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the Node
* **body**: [Node](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([Node](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node)): OK
201 ([Node](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the Node
* **body**: [Patch](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/patch/#Patch), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **force** (*in query*): boolean
[force](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#force)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)