---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#32-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 92
summary: [LegacyServiceAccountTokenNoAutoGeneration feature gate](/docs/reference/command-line-tools-reference/feature-gates-removed/) (which was enabled by default from Kubernetes v1.24 to v1.26), prevented...
---

[LegacyServiceAccountTokenNoAutoGeneration feature gate](/docs/reference/command-line-tools-reference/feature-gates-removed/)
(which was enabled by default from Kubernetes v1.24 to v1.26), prevented Kubernetes from automatically creating these tokens for
ServiceAccounts. The feature gate is removed in v1.27, because it was elevated to GA status; you can still create indefinite service account tokens manually, but should take into account the security implications.