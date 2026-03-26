---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#16-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 475
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