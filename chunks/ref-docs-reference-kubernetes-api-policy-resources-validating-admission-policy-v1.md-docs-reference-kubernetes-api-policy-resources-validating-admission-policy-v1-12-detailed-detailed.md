---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#12-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 974
summary: * **spec.matchResources.objectSelector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector)) ObjectSelector decides whether to run...
---

* **spec.matchResources.objectSelector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
ObjectSelector decides whether to run the validation based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the cel validation, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.
* **spec.matchResources.resourceRules** ([]NamedRuleWithOperations)
*Atomic: will be replaced during a merge*
ResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy matches. The policy cares about an operation if it matches *any* Rule.
*NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.*
* **spec.matchResources.resourceRules.apiGroups** ([]string)
*Atomic: will be replaced during a merge*
APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
* **spec.matchResources.resourceRules.apiVersions** ([]string)
*Atomic: will be replaced during a merge*
APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
* **spec.matchResources.resourceRules.operations** ([]string)
*Atomic: will be replaced during a merge*
Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or \* for all of those operations and any future admission operations that are added. If '\*' is present, the length of the slice must be one. Required.
* **spec.matchResources.resourceRules.resourceNames** ([]string)
*Atomic: will be replaced during a merge*
ResourceNames is an optional white list of names that the rule applies to. An empty set means that everything is allowed.
* **spec.matchResources.resourceRules.resources** ([]string)
*Atomic: will be replaced during a merge*
Resources is a list of resources this rule applies to.
For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/\*' means all resources and their subresources.
If wildcard is present, the validation rule will ensure resources do not overlap with each other.
Depending on the enclosing object, subresources might not be allowed. Required.
* **spec.matchResources.resourceRules.scope** (string)
scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "\*".
Possible enum values:
* `"\*"` means that all scopes are included.
* `"Cluster"` means that scope is limited to cluster-scoped objects. Namespace objects are cluster-scoped.
* `"Namespaced"` means that scope is limited to namespaced objects.
* **spec.paramRef** (ParamRef)
paramRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in ParamKind of the bound ValidatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the ValidatingAdmissionPolicy applied. If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.
*ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.*
* **spec.paramRef.name** (string)
name is the name of the resource being referenced.
One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
A single parameter used for all admission requests can be configured by setting the `name` field, leaving `selector` blank, and setting namespace if `paramKind` is namespace-scoped.