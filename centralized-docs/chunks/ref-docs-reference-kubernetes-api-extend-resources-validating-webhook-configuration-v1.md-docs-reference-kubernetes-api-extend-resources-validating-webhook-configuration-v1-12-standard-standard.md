---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#12-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingWebhookConfigurationList
token_count: 405
summary: ## ValidatingWebhookConfigurationList ValidatingWebhookConfigurationList is a list of ValidatingWebhookConfiguration. * **items**...
---

## ValidatingWebhookConfigurationList
ValidatingWebhookConfigurationList is a list of ValidatingWebhookConfiguration.
* **items** ([][ValidatingWebhookConfiguration](https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/validating-webhook-configuration-v1/#ValidatingWebhookConfiguration)), required
List of ValidatingWebhookConfiguration.
* **apiVersion** (string)
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources)
* **kind** (string)
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
#### Parameters
* **name** (*in path*): string, required
name of the ValidatingWebhookConfiguration
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)