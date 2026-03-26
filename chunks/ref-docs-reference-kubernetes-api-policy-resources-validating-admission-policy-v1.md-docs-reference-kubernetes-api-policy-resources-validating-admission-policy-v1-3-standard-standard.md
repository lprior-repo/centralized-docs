---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#3-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 392
summary: * **apiVersion**: admissionregistration.k8s.io/v1 * **kind**: ValidatingAdmissionPolicy * **metadata**...
---

* **apiVersion**: admissionregistration.k8s.io/v1
* **kind**: ValidatingAdmissionPolicy
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object metadata; More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata).
* **spec** (ValidatingAdmissionPolicySpec)
Specification of the desired behavior of the ValidatingAdmissionPolicy.
*ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.*
* **spec.auditAnnotations** ([]AuditAnnotation)
*Atomic: will be replaced during a merge*
auditAnnotations contains CEL expressions which are used to produce audit annotations for the audit event of the API request. validations and auditAnnotations may not both be empty; a least one of validations or auditAnnotations is required.
*AuditAnnotation describes how to produce an audit annotation for an API request.*
* **spec.auditAnnotations.key** (string), required
key specifies the audit annotation key. The audit annotation keys of a ValidatingAdmissionPolicy must be unique. The key must be a qualified name ([A-Za-z0-9][-A-Za-z0-9\_.]\*) no more than 63 bytes in length.
The key is combined with the resource name of the ValidatingAdmissionPolicy to construct an audit annotation key: "{ValidatingAdmissionPolicy name}/{key}".
If an admission webhook uses the same resource name as this ValidatingAdmissionPolicy and the same audit annotation key, the annotation key will be identical. In this case, the first annotation written with the key will be included in the audit event and all subsequent annotations with the same key will be discarded.
Required.