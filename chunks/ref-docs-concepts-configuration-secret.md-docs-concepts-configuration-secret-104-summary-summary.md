---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#104-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 128
summary: A container using a Secret as a [subPath](/docs/concepts/storage/volumes/#using-subpath) volume mount does not receive automated Secret updates. The kubelet keeps a cache of the current keys and...
---

A container using a Secret as a
[subPath](/docs/concepts/storage/volumes/#using-subpath) volume mount does not receive
automated Secret updates.
The kubelet keeps a cache of the current keys and values for the Secrets that are used in
volumes for pods on that node.
You can configure the way that the kubelet detects changes from the cached values. The
`configMapAndSecretChangeDetectionStrategy` field in the
[kubelet configuration](/docs/reference/config-api/kubelet-config.v1beta1/) controls
which strategy the kubelet uses. The default strategy is `Watch`.