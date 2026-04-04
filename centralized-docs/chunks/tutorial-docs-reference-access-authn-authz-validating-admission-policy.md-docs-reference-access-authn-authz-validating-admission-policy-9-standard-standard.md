---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#9-standard
chunk_level: standard
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 411
summary: #### Optional parameters It can be convenient to be able to have optional parameters as part of a parameter resource, and only validate them if present. CEL provides `has()`, which checks if the key...
---

#### Optional parameters
It can be convenient to be able to have optional parameters as part of a parameter resource, and
only validate them if present. CEL provides `has()`, which checks if the key passed to it exists.
CEL also implements Boolean short-circuiting. If the first half of a logical OR evaluates to true,
it won’t evaluate the other half (since the result of the entire OR will be true regardless).
Combining the two, we can provide a way to validate optional parameters:
`!has(params.optionalNumber) || (params.optionalNumber &gt;= 5 &amp;&amp; params.optionalNumber &lt;= 10)`
Here, we first check that the optional parameter is present with `!has(params.optionalNumber)`.
* If `optionalNumber` hasn’t been defined, then the expression short-circuits since
`!has(params.optionalNumber)` will evaluate to true.
* If `optionalNumber` has been defined, then the latter half of the CEL expression will be
evaluated, and optionalNumber will be checked to ensure that it contains a value between 5 and
10 inclusive.#### Per-namespace Parameters
As the author of a ValidatingAdmissionPolicy and its ValidatingAdmissionPolicyBinding,
you can choose to specify cluster-wide, or per-namespace parameters.
If you specify a `namespace` for the binding's `paramRef`, the control plane only
searches for parameters in that namespace.
However, if `namespace` is not specified in the ValidatingAdmissionPolicyBinding, the
API server can search for relevant parameters in the namespace that a request is against.
For example, if you make a request to modify a ConfigMap in the `default` namespace and
there is a relevant ValidatingAdmissionPolicyBinding with no `namespace` set, then the
API server looks for a parameter object in `default`.
This design enables policy configuration that depends on the namespace
of the resource being manipulated, for more fine-tuned control.