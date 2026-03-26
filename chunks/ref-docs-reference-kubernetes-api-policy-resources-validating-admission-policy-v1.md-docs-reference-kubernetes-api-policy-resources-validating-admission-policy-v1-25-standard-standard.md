---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#25-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 470
summary: * **spec.matchResources.resourceRules.scope** (string) scope specifies the scope of this rule. Valid values are \"Cluster\", \"Namespaced\", and \"*\" \"Cluster\" means that only cluster-scoped resources...
---

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
* **spec.paramRef.namespace** (string)
namespace is the namespace of the referenced resource. Allows limiting the search for params to a specific namespace. Applies to both `name` and `selector` fields.
A per-namespace parameter may be used by specifying a namespace-scoped `paramKind` in the policy and leaving this field empty.
* If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error.