---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#22-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 442
summary: Type Checking has the following limitation: * No wildcard matching. If `spec.matchConstraints.resourceRules` contains `\"\*\"` in any of `apiGroups`, `apiVersions` or `resources`, the types that `\"\*\"`...
---

Type Checking has the following limitation:
* No wildcard matching. If `spec.matchConstraints.resourceRules` contains `"\*"` in any of `apiGroups`, `apiVersions` or `resources`,
the types that `"\*"` matches will not be checked.
* The number of matched types is limited to 10. This is to prevent a policy that manually specifying too many types.
to consume excessive computing resources. In the order of ascending group, version, and then resource, 11th combination and beyond are ignored.
* Type Checking does not affect the policy behavior in any way. Even if the type checking detects errors, the policy will continue
to evaluate. If errors do occur during evaluate, the failure policy will decide its outcome.
* Type Checking does not apply to CRDs, including matched CRD types and reference of paramKind. The support for CRDs will come in future release.### Variable composition
If an expression grows too complicated, or part of the expression is reusable and computationally expensive to evaluate,
you can extract some part of the expressions into variables. A variable is a named expression that can be referred later
in `variables` in other expressions.
```
`spec:
variables:
- name: foo
expression: "'foo' in object.spec.metadata.labels ? object.spec.metadata.labels['foo'] : 'default'"
validations:
- expression: variables.foo == 'bar'
`
```
A variable is lazily evaluated when it is first referred. Any error that occurs during the evaluation will be
reported during the evaluation of the referring expression. Both the result and potential error are memorized and
count only once towards the runtime cost.
The order of variables are important because a variable can refer to other variables that are defined before it.
This ordering prevents circular references.
The following is a more complex example of enforcing that image repo names match the environment defined in its namespace.
[`access/image-matches-namespace-environment.policy.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/image-matches-namespace-environment.policy.yaml)![](/images/copycode.svg "Copy access/image-matches-namespace-environment.policy.yaml to clipboard")