---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#18-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 63
summary: * [Allow the kubelet to pull container images from private registries](/docs/tasks/configure-pod-container/pull-image-private-registry/). The Kubernetes control plane also uses Secrets; for example,...
---

* [Allow the kubelet to pull container images from private registries](/docs/tasks/configure-pod-container/pull-image-private-registry/).
The Kubernetes control plane also uses Secrets; for example,
[bootstrap token Secrets](#bootstrap-token-secrets) are a mechanism to
help automate node registration.