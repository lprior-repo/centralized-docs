---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#12-standard
chunk_level: standard
chunk_type: prose
heading: NetworkPolicySpec
token_count: 511
summary: #### Parameters * **namespace** (*in path*): string, required [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace) * **body**:...
---

#### Parameters
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): OK
201 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): Created
202 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): Accepted
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the NetworkPolicy
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)