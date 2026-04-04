---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#59-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 105
summary: * `Fail` means that an error calling the ValidatingAdmissionPolicy causes the admission to fail and the API request to be rejected. Note that the `failurePolicy` is defined inside...
---

* `Fail` means that an error calling the ValidatingAdmissionPolicy causes the admission to fail
and the API request to be rejected.
Note that the `failurePolicy` is defined inside `ValidatingAdmissionPolicy`:
[`validatingadmissionpolicy/failure-policy-ignore.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/failure-policy-ignore.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/failure-policy-ignore.yaml to clipboard")