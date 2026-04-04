---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#17-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 94
summary: * populating a configuration file based on a [ConfigMap](/docs/concepts/configuration/configmap/) or a [Secret](/docs/concepts/configuration/secret/) * providing some temporary scratch space for a...
---

* populating a configuration file based on a [ConfigMap](/docs/concepts/configuration/configmap/)
or a [Secret](/docs/concepts/configuration/secret/)
* providing some temporary scratch space for a Pod
* sharing a filesystem between two different containers in the same Pod
* sharing a filesystem between two different Pods (even if those Pods run on different nodes)
* durably storing data so that it stays available even if the Pod restarts or is replaced