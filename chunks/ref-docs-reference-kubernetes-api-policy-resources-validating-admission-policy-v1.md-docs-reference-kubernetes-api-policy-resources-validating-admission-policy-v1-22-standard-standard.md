---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#22-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 439
summary: * **spec.matchResources.excludeResourceRules.scope** (string) scope specifies the scope of this rule. Valid values are \"Cluster\", \"Namespaced\", and \"*\" \"Cluster\" means that only cluster-scoped...
---

* **spec.matchResources.excludeResourceRules.scope** (string)
scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "\*".
Possible enum values:
* `"\*"` means that all scopes are included.
* `"Cluster"` means that scope is limited to cluster-scoped objects. Namespace objects are cluster-scoped.
* `"Namespaced"` means that scope is limited to namespaced objects.
* **spec.matchResources.matchPolicy** (string)
matchPolicy defines how the "MatchResources" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".
* Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but "rules" only included `apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the ValidatingAdmissionPolicy.
* Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and "rules" only included `apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the ValidatingAdmissionPolicy.
Defaults to "Equivalent"
Possible enum values:
* `"Equivalent"` means requests should be sent to the webhook if they modify a resource listed in rules via another API group or version.
* `"Exact"` means requests should only be sent to the webhook if they exactly match a given rule.