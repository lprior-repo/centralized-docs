---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#31-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 82
summary: [operator](/docs/concepts/extend-kubernetes/operator/) that fetches short-lived session tokens from an external service, and then creates Secrets based on those short-lived session tokens. Pods...
---

[operator](/docs/concepts/extend-kubernetes/operator/)
that fetches short-lived session tokens from an external service, and then creates Secrets based
on those short-lived session tokens. Pods running in your cluster can make use of the session tokens,
and operator ensures they are valid. This separation means that you can run Pods that are unaware of
the exact mechanisms for issuing and refreshing those session tokens.