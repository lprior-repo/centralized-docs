---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-service-account-v1.md/docs-reference-kubernetes-api-authentication-resources-service-account-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-service-account-v1.md/docs-reference-kubernetes-api-authentication-resources-service-account-v1#10-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount
token_count: 118
summary: \"kubernetes.io/enforce-mountable-secrets\" annotation set to \"true\". The \"kubernetes.io/enforce-mountable-secrets\" annotation is deprecated since v1.32. Prefer separate namespaces to isolate access to...
---

"kubernetes.io/enforce-mountable-secrets" annotation set to "true". The "kubernetes.io/enforce-mountable-secrets" annotation is deprecated since v1.32. Prefer separate namespaces to isolate access to mounted secrets. This field should not be used to find auto-generated service account token secrets for use outside of pods. Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created. More info: [https://kubernetes.io/docs/concepts/configuration/secret](https://kubernetes.io/docs/concepts/configuration/secret)