---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#4-detailed
chunk_level: detailed
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 935
summary: ``` `apiVersion: admissionregistration.k8s.io/v1 kind: ValidatingAdmissionPolicyBinding metadata: name: \"replicalimit-binding-nontest\" spec: policyName: \"replicalimit-policy.example.com\"...
---

```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: "replicalimit-binding-nontest"
spec:
policyName: "replicalimit-policy.example.com"
validationActions: [Deny]
paramRef:
name: "replica-limit-prod.example.com"
namespace: "default"
parameterNotFoundAction: Deny
matchResources:
namespaceSelector:
matchExpressions:
- key: environment
operator: NotIn
values:
- test
`
```
Notice this binding applies a different parameter to resources which
are not in the `test` environment.
And have a parameter resource:
[`validatingadmissionpolicy/replicalimit-param-prod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/replicalimit-param-prod.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/replicalimit-param-prod.yaml to clipboard")
```
`apiVersion: rules.example.com/v1
kind: ReplicaLimit
metadata:
name: "replica-limit-prod.example.com"
maxReplicas: 100`
```
For each admission request, the API server evaluates CEL expressions of each
(policy, binding, param) combination that match the request. For a request
to be admitted it must pass **all** evaluations.
If multiple bindings match the request, the policy will be evaluated for each,
and they must all pass evaluation for the policy to be considered passed.
If multiple parameters match a single binding, the policy rules will be evaluated
for each param, and they too must all pass for the binding to be considered passed.
Bindings can have overlapping match criteria. The policy is evaluated for each
matching binding-parameter combination. A policy may even be evaluated multiple
times if multiple bindings match it, or a single binding that matches multiple
parameters.
The params object representing a parameter resource will not be set if a parameter resource has
not been bound, so for policies requiring a parameter resource, it can be useful to add a check to
ensure one has been bound. A parameter resource will not be bound and `params` will be null
if `paramKind` of the policy, or `paramRef` of the binding are not specified.
For the use cases requiring parameter configuration, we recommend to add a param check in
`spec.validations[0].expression`:
```
`- expression: "params != null"
message: "params missing but required to bind to this policy"
`
```
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