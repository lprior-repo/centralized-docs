---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#39-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 128
summary: 1. The name of each Secret that is mounted as a volume in a Pod must appear in the `secrets` field of the Pod's ServiceAccount. 2. The name of each Secret referenced using `envFrom` in a Pod must...
---

1. The name of each Secret that is mounted as a volume in a Pod must appear in the `secrets` field of the
Pod's ServiceAccount.
2. The name of each Secret referenced using `envFrom` in a Pod must also appear in the `secrets`
field of the Pod's ServiceAccount.
3. The name of each Secret referenced using `imagePullSecrets` in a Pod must also appear in the `secrets`
field of the Pod's ServiceAccount.
By understanding and enforcing these restrictions, cluster administrators can maintain a tighter security profile and ensure that secrets are accessed only by the appropriate resources.