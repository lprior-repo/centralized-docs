---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#100-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 89
summary: If multiple resources are matched in `spec.matchConstraints`, all of matched resources will be checked against. For example, the following policy definition...
---

If multiple resources are matched in `spec.matchConstraints`, all of matched resources will be checked against.
For example, the following policy definition
[`validatingadmissionpolicy/typechecking-multiple-match.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/typechecking-multiple-match.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/typechecking-multiple-match.yaml to clipboard")