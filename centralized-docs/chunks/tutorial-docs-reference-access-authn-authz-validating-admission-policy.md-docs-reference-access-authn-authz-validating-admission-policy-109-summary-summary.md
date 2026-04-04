---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#109-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 85
summary: This ordering prevents circular references. The following is a more complex example of enforcing that image repo names match the environment defined in its namespace....
---

This ordering prevents circular references.
The following is a more complex example of enforcing that image repo names match the environment defined in its namespace.
[`access/image-matches-namespace-environment.policy.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/image-matches-namespace-environment.policy.yaml)![](/images/copycode.svg "Copy access/image-matches-namespace-environment.policy.yaml to clipboard")