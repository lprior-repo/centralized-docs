---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#8-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingAdmissionPolicyList
token_count: 923
summary: * **status.conditions** ([]Condition) *Map: unique values on key type will be kept during a merge* The conditions represent the latest available observations of a policy's current state. *Condition...
---

* **status.conditions** ([]Condition)
*Map: unique values on key type will be kept during a merge*
The conditions represent the latest available observations of a policy's current state.
*Condition contains details for one aspect of the current state of this API Resource.*
* **status.conditions.lastTransitionTime** (Time), required
lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **status.conditions.message** (string), required
message is a human readable message indicating details about the transition. This may be an empty string.
* **status.conditions.reason** (string), required
reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
* **status.conditions.status** (string), required
status of the condition, one of True, False, Unknown.
* **status.conditions.type** (string), required
type of condition in CamelCase or in foo.example.com/CamelCase.
* **status.conditions.observedGeneration** (int64)
observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
* **status.observedGeneration** (int64)
The generation observed by the controller.
* **status.typeChecking** (TypeChecking)
The results of type checking for each expression. Presence of this field indicates the completion of the type checking.
*TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy*
* **status.typeChecking.expressionWarnings** ([]ExpressionWarning)
*Atomic: will be replaced during a merge*
The type checking warnings for each expression.
*ExpressionWarning is a warning information that targets a specific expression.*
* **status.typeChecking.expressionWarnings.fieldRef** (string), required
The path to the field that refers the expression. For example, the reference to the expression of the first item of validations is "spec.validations[0].expression"
* **status.typeChecking.expressionWarnings.warning** (string), required
The content of type checking information in a human-readable form. Each line of the warning contains the type that the expression is checked against, followed by the type check error from the compiler.
## ValidatingAdmissionPolicyList
ValidatingAdmissionPolicyList is a list of ValidatingAdmissionPolicy.
* **items** ([][ValidatingAdmissionPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/validating-admission-policy-binding-v1/#ValidatingAdmissionPolicy)), required
List of ValidatingAdmissionPolicy.
* **apiVersion** (string)
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources)
* **kind** (string)
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)