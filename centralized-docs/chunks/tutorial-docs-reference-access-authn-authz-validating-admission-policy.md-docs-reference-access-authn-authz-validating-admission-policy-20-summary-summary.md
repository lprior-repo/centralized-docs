---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#20-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 126
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