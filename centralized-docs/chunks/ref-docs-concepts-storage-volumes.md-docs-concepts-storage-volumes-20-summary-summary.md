---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#20-summary
chunk_level: summary
chunk_type: prose
heading: Why volumes are important
token_count: 85
summary: * **Data persistence:** On-disk files in a container are ephemeral, which presents some problems for non-trivial applications when running in containers. One problem occurs when a container crashes...
---

* **Data persistence:** On-disk files in a container are ephemeral, which presents some problems for
non-trivial applications when running in containers. One problem occurs when
a container crashes or is stopped; the container state is not saved, so all of the
files that were created or modified during the lifetime of the container are lost.
After a crash, kubelet restarts the container with a clean state.