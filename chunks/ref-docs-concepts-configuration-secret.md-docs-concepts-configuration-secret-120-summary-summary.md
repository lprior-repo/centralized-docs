---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#120-summary
chunk_level: summary
chunk_type: prose
heading: Information security for Secrets
token_count: 119
summary: Although ConfigMap and Secret work similarly, Kubernetes applies some additional protection for Secret objects. Secrets often hold values that span a spectrum of importance, many of which can cause...
---

Although ConfigMap and Secret work similarly, Kubernetes applies some additional
protection for Secret objects.
Secrets often hold values that span a spectrum of importance, many of which can
cause escalations within Kubernetes (e.g. service account tokens) and to
external systems. Even if an individual app can reason about the power of the
Secrets it expects to interact with, other apps within the same namespace can
render those assumptions invalid.
Authorization configuration affects how Secret data can be accessed within a namespace.
For example, granting **list** or **watch** permissions on Secrets allows a subject