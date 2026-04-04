---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#7-standard
chunk_level: standard
chunk_type: prose
heading: StorageClass
token_count: 340
summary: #### Response 200 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): OK 201...
---

#### Response
200 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): OK
201 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the StorageClass
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
#### Response
200 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): OK
201 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): Created
401: Unauthorized