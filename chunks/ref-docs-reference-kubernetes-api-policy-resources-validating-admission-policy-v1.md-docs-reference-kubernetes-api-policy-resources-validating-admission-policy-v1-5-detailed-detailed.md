---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#5-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 977
summary: * **spec.matchConstraints.objectSelector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector)) ObjectSelector decides whether to...
---

* **spec.matchConstraints.objectSelector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
ObjectSelector decides whether to run the validation based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the cel validation, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.
* **spec.matchConstraints.resourceRules** ([]NamedRuleWithOperations)
*Atomic: will be replaced during a merge*
ResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy matches. The policy cares about an operation if it matches *any* Rule.
*NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.*
* **spec.matchConstraints.resourceRules.apiGroups** ([]string)
*Atomic: will be replaced during a merge*
APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
* **spec.matchConstraints.resourceRules.apiVersions** ([]string)
*Atomic: will be replaced during a merge*
APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
* **spec.matchConstraints.resourceRules.operations** ([]string)
*Atomic: will be replaced during a merge*
Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or \* for all of those operations and any future admission operations that are added. If '\*' is present, the length of the slice must be one. Required.
* **spec.matchConstraints.resourceRules.resourceNames** ([]string)
*Atomic: will be replaced during a merge*
ResourceNames is an optional white list of names that the rule applies to. An empty set means that everything is allowed.
* **spec.matchConstraints.resourceRules.resources** ([]string)
*Atomic: will be replaced during a merge*
Resources is a list of resources this rule applies to.
For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/\*' means all resources and their subresources.
If wildcard is present, the validation rule will ensure resources do not overlap with each other.
Depending on the enclosing object, subresources might not be allowed. Required.
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