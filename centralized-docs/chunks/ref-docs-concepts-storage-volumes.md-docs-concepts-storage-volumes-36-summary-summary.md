---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#36-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 113
summary: * holding files that a content-manager container fetches while a webserver container serves the data The `emptyDir.medium` field controls where `emptyDir` volumes are stored. By default `emptyDir`...
---

* holding files that a content-manager container fetches while a webserver
container serves the data
The `emptyDir.medium` field controls where `emptyDir` volumes are stored. By
default `emptyDir` volumes are stored on whatever medium that backs the node
such as disk, SSD, or network storage, depending on your environment. If you set
the `emptyDir.medium` field to `"Memory"`, Kubernetes mounts a tmpfs (RAM-backed
filesystem) for you instead. While tmpfs is very fast, be aware that, unlike