---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#15-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 465
summary: * **spec.validations.reason** (string) Reason represents a machine-readable description of why this validation failed. If this is the first validation in the list to fail, this reason, as well as the...
---

* **spec.validations.reason** (string)
Reason represents a machine-readable description of why this validation failed. If this is the first validation in the list to fail, this reason, as well as the corresponding HTTP response code, are used in the HTTP response to the client. The currently supported reasons are: "Unauthorized", "Forbidden", "Invalid", "RequestEntityTooLarge". If not set, StatusReasonInvalid is used in the response to the client.
* **spec.variables** ([]Variable)
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
Variables contain definitions of variables that can be used in composition of other expressions. Each variable is defined as a named CEL expression. The variables defined here will be available under `variables` in other expressions of the policy except MatchConditions because MatchConditions are evaluated before the rest of the policy.
The expression of a variable can refer to other variables defined earlier in the list but not those after. Thus, Variables must be sorted by the order of first appearance and acyclic.
*Variable is the definition of a variable that is used for composition. A variable is defined as a named expression.*
* **spec.variables.expression** (string), required
Expression is the expression that will be evaluated as the value of the variable. The CEL expression has access to the same identifiers as the CEL expressions in Validation.
* **spec.variables.name** (string), required
Name is the name of the variable. The name must be a valid CEL identifier and unique among all variables. The variable can be accessed in other expressions through `variables` For example, if name is "foo", the variable will be available as `variables.foo`
* **status** (ValidatingAdmissionPolicyStatus)
The status of the ValidatingAdmissionPolicy, including warnings that are useful to determine if the policy behaves in the expected way. Populated by the system. Read-only.
*ValidatingAdmissionPolicyStatus represents the status of an admission validation policy.*
* **status.conditions** ([]Condition)
*Map: unique values on key type will be kept during a merge*
The conditions represent the latest available observations of a policy's current state.
*Condition contains details for one aspect of the current state of this API Resource.*