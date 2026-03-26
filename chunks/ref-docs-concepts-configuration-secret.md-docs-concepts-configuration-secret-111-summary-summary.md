---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#111-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 60
summary: ### Container image pull Secrets If you want to fetch container images from a private repository, you need a way for the kubelet on each node to authenticate to that repository. You can configure...
---

### Container image pull Secrets
If you want to fetch container images from a private repository, you need a way for
the kubelet on each node to authenticate to that repository. You can configure
*image pull Secrets* to make this possible. These Secrets are configured at the Pod
level.