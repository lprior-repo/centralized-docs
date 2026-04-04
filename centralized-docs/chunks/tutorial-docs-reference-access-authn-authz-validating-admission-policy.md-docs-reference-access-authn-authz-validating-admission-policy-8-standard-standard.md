---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#8-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 320
summary: ``` `apiVersion: rules.example.com/v1 kind: ReplicaLimit metadata: name: \"replica-limit-prod.example.com\" maxReplicas: 100` ``` For each admission request, the API server evaluates CEL expressions of...
---

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