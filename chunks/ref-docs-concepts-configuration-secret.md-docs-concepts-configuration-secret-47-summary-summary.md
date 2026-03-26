---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#47-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 102
summary: After creating the Secret, wait for Kubernetes to populate the `token` key in the `data` field. See the [ServiceAccount](/docs/concepts/security/service-accounts/) documentation for more information...
---

After creating the Secret, wait for Kubernetes to populate the `token` key in the `data` field.
See the [ServiceAccount](/docs/concepts/security/service-accounts/)
documentation for more information on how ServiceAccounts work.
You can also check the `automountServiceAccountToken` field and the
`serviceAccountName` field of the
[`Pod`](/docs/reference/generated/kubernetes-api/v1.35/#pod-v1-core)
for information on referencing ServiceAccount credentials from within Pods.