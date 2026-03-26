---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#31-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 128
summary: * [Service Account Token Secrets](/docs/tasks/configure-pod-container/configure-service-account/#manually-create-an-api-token-for-a-serviceaccount) (not recommended): You can mount service account...
---

* [Service Account Token Secrets](/docs/tasks/configure-pod-container/configure-service-account/#manually-create-an-api-token-for-a-serviceaccount)
(not recommended): You can mount service account tokens as Kubernetes
Secrets in Pods. These tokens don't expire and don't rotate. In versions prior to v1.24, a permanent token was automatically created for each service account.
This method is not recommended anymore, especially at scale, because of the risks associated
with static, long-lived credentials. The [LegacyServiceAccountTokenNoAutoGeneration feature gate](/docs/reference/command-line-tools-reference/feature-gates-removed/)