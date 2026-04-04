---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#75-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 127
summary: `IfNotPresent`The kubelet pulls if the reference isn't already present on disk. The Pod becomes `Failed` if the reference isn't present and the pull fails. The volume gets re-resolved if the Pod gets...
---

`IfNotPresent`The kubelet pulls if the reference isn't already present on disk. The Pod becomes
`Failed` if the reference isn't present and the pull fails.
The volume gets re-resolved if the Pod gets deleted and recreated, which means
that new remote content will become available on Pod recreation. A failure to
resolve or pull the image during Pod startup will block containers from starting
and may add significant latency. Failures will be retried using normal volume
backoff and will be reported on the Pod reason and message.
The types of objects that may be mounted by this volume are defined by the