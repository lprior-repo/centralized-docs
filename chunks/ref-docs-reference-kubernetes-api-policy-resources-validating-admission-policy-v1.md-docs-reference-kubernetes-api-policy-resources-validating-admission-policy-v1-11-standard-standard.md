---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#11-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 447
summary: * **spec.matchConstraints.resourceRules.scope** (string) scope specifies the scope of this rule. Valid values are \"Cluster\", \"Namespaced\", and \"*\" \"Cluster\" means that only cluster-scoped resources...
---

* **spec.matchConstraints.resourceRules.scope** (string)
scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "\*".
Possible enum values:
* `"\*"` means that all scopes are included.
* `"Cluster"` means that scope is limited to cluster-scoped objects. Namespace objects are cluster-scoped.
* `"Namespaced"` means that scope is limited to namespaced objects.
* **spec.paramKind** (ParamKind)
ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.
*ParamKind is a tuple of Group Kind and Version.*
* **spec.paramKind.apiVersion** (string)
APIVersion is the API group version the resources belong to. In format of "group/version". Required.
* **spec.paramKind.kind** (string)
Kind is the API kind the resources belong to. Required.
* **spec.validations** ([]Validation)
*Atomic: will be replaced during a merge*
Validations contain CEL expressions which is used to apply the validation. Validations and AuditAnnotations may not both be empty; a minimum of one Validations or AuditAnnotations is required.
*Validation specifies the CEL expression which is used to apply the validation.*
* **spec.validations.expression** (string), required
Expression represents the expression which will be evaluated by CEL. ref: [https://github.com/google/cel-spec](https://github.com/google/cel-spec) CEL expressions have access to the contents of the API request/response, organized into CEL variables as well as some other useful variables: