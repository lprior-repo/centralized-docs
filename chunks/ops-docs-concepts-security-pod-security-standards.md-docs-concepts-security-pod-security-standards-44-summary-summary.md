---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#44-summary
chunk_level: summary
chunk_type: prose
heading: Policy Instantiation
token_count: 84
summary: ## Policy Instantiation Decoupling policy definition from policy instantiation allows for a common understanding and consistent language of policies across clusters, independent of the underlying...
---

## Policy Instantiation
Decoupling policy definition from policy instantiation allows for a common understanding and
consistent language of policies across clusters, independent of the underlying enforcement
mechanism.
As mechanisms mature, they will be defined below on a per-policy basis. The methods of enforcement
of individual policies are not defined here.
[**Pod Security Admission Controller**](/docs/concepts/security/pod-security-admission/)