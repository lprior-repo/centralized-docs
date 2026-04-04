---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#11-summary
chunk_level: summary
chunk_type: prose
heading: What Resources Make a Policy
token_count: 114
summary: * The `ValidatingAdmissionPolicy` describes the abstract logic of a policy (think: \"this policy makes sure a particular label is set to a particular value\"). * A parameter resource provides...
---

* The `ValidatingAdmissionPolicy` describes the abstract logic of a policy
(think: "this policy makes sure a particular label is set to a particular value").
* A parameter resource provides information to a ValidatingAdmissionPolicy to make it a concrete
statement (think "the `owner` label must be set to something that ends in `.company.com`").
A native type such as ConfigMap or a CRD defines the schema of a parameter resource.
`ValidatingAdmissionPolicy` objects specify what Kind they are expecting for their parameter resource.