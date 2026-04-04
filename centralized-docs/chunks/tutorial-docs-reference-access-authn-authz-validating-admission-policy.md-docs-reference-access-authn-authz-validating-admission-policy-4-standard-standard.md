---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#4-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 278
summary: #### Validation actions Each `ValidatingAdmissionPolicyBinding` must specify one or more `validationActions` to declare how `validations` of a policy are enforced. The supported `validationActions`...
---

#### Validation actions
Each `ValidatingAdmissionPolicyBinding` must specify one or more
`validationActions` to declare how `validations` of a policy are enforced.
The supported `validationActions` are:
* `Deny`: Validation failure results in a denied request.
* `Warn`: Validation failure is reported to the request client
as a [warning](/blog/2020/09/03/warnings/).
* `Audit`: Validation failure is included in the audit event for the API request.
For example, to both warn clients about a validation failure and to audit the
validation failures, use:
```
`validationActions: [Warn, Audit]
`
```
`Deny` and `Warn` may not be used together since this combination
needlessly duplicates the validation failure both in the
API response body and the HTTP warning headers.
A `validation` that evaluates to false is always enforced according to these
actions. Failures defined by the `failurePolicy` are enforced
according to these actions only if the `failurePolicy` is set to `Fail` (or not specified),
otherwise the failures are ignored.
See [Audit Annotations: validation failures](/docs/reference/labels-annotations-taints/audit-annotations/#validation-policy-admission-k8s-io-validation-failure)
for more details about the validation failure audit annotation.