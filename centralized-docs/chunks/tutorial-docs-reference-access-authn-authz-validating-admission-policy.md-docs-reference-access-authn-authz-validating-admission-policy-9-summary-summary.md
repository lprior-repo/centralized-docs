---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#9-summary
chunk_level: summary
chunk_type: prose
heading: What is Validating Admission Policy?
token_count: 108
summary: # Validating Admission Policy FEATURE STATE: `Kubernetes v1.30 [stable]` This page provides an overview of Validating Admission Policy. ## What is Validating Admission Policy? Validating admission...
---

# Validating Admission Policy
FEATURE STATE:
`Kubernetes v1.30 [stable]`
This page provides an overview of Validating Admission Policy.
## What is Validating Admission Policy?
Validating admission policies offer a declarative, in-process alternative to validating admission webhooks.
Validating admission policies use the Common Expression Language (CEL) to declare the validation
rules of a policy.
Validation admission policies are highly configurable, enabling policy authors to define policies
that can be parameterized and scoped to resources as needed by cluster administrators.