---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#31-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 106
summary: #### Note: * You must [create a ConfigMap](/docs/tasks/configure-pod-container/configure-pod-configmap/#create-a-configmap) before you can use it. * A ConfigMap is always mounted as `readOnly`. * A...
---

#### Note:
* You must [create a ConfigMap](/docs/tasks/configure-pod-container/configure-pod-configmap/#create-a-configmap)
before you can use it.
* A ConfigMap is always mounted as `readOnly`.
* A container using a ConfigMap as a [`subPath`](#using-subpath) volume mount will not
receive updates when the ConfigMap changes.
* Text data is exposed as files using the UTF-8 character encoding.
For other character encodings, use `binaryData`.