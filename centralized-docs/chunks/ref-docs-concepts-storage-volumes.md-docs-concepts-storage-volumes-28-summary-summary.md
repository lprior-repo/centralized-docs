---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#28-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 128
summary: ### configMap A [ConfigMap](/docs/tasks/configure-pod-container/configure-pod-configmap/) provides a way to inject configuration data into Pods. The data stored in a ConfigMap can be referenced in a...
---

### configMap
A [ConfigMap](/docs/tasks/configure-pod-container/configure-pod-configmap/)
provides a way to inject configuration data into Pods.
The data stored in a ConfigMap can be referenced in a volume of type
`configMap` and then consumed by containerized applications running in a Pod.
When referencing a ConfigMap, you provide the name of the ConfigMap in the
volume. You can customize the path to use for a specific
entry in the ConfigMap. The following configuration shows how to mount
the `log-config` ConfigMap onto a Pod called `configmap-pod`: