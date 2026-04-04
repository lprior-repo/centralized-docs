---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#141-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 126
summary: [`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html) #### Warning: `Bidirectional` mount propagation can be dangerous. It can damage the host operating system, and therefore, it is...
---

[`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html)
#### Warning:
`Bidirectional` mount propagation can be dangerous. It can damage
the host operating system, and therefore, it is allowed only in privileged
containers. Familiarity with Linux kernel behavior is strongly recommended.
In addition, any volume mounts created by containers in Pods must be destroyed
(unmounted) by the containers on termination.
## Read-only mounts
A mount can be made read-only by setting the `.spec.containers[\*].volumeMounts[\*].readOnly`
field to `true`.