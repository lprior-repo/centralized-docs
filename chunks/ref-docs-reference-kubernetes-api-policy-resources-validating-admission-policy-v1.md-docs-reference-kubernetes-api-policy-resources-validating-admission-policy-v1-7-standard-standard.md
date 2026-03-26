---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#7-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 460
summary: * **spec.matchConstraints.excludeResourceRules.resourceNames** ([]string) *Atomic: will be replaced during a merge* ResourceNames is an optional white list of names that the rule applies to. An empty...
---

* **spec.matchConstraints.excludeResourceRules.resourceNames** ([]string)
*Atomic: will be replaced during a merge*
ResourceNames is an optional white list of names that the rule applies to. An empty set means that everything is allowed.
* **spec.matchConstraints.excludeResourceRules.resources** ([]string)
*Atomic: will be replaced during a merge*
Resources is a list of resources this rule applies to.
For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/\*' means all resources and their subresources.
If wildcard is present, the validation rule will ensure resources do not overlap with each other.
Depending on the enclosing object, subresources might not be allowed. Required.
* **spec.matchConstraints.excludeResourceRules.scope** (string)
scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "\*".
Possible enum values:
* `"\*"` means that all scopes are included.
* `"Cluster"` means that scope is limited to cluster-scoped objects. Namespace objects are cluster-scoped.
* `"Namespaced"` means that scope is limited to namespaced objects.
* **spec.matchConstraints.matchPolicy** (string)
matchPolicy defines how the "MatchResources" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".
* Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but "rules" only included `apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the ValidatingAdmissionPolicy.