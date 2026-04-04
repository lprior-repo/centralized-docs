---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#34-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 112
summary: This policy parameter resource limits deployments to a max of 3 replicas. An admission policy may have multiple bindings. To bind all other environments to have a maxReplicas limit of 100, create...
---

This policy parameter resource limits deployments to a max of 3 replicas.
An admission policy may have multiple bindings. To bind all other environments
to have a maxReplicas limit of 100, create another ValidatingAdmissionPolicyBinding:
[`validatingadmissionpolicy/binding-with-param-prod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/binding-with-param-prod.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/binding-with-param-prod.yaml to clipboard")