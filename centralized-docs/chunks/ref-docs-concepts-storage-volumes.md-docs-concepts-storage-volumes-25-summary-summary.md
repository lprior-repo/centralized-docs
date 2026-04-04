---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#25-summary
chunk_level: summary
chunk_type: prose
heading: How volumes work
token_count: 122
summary: `.spec.volumes` and declare where to mount those volumes into containers in `.spec.containers[\*].volumeMounts`. When a Pod is launched, a process in the container sees a filesystem view composed...
---

`.spec.volumes`
and declare where to mount those volumes into containers in `.spec.containers[\*].volumeMounts`.
When a Pod is launched, a process in the container sees a filesystem view composed from the initial contents of
the [container image](/docs/reference/glossary/?all=true#term-image), plus volumes
(if defined) mounted inside the container.
The process sees a root filesystem that initially matches the contents of the container image.
Any writes to within that filesystem hierarchy, if allowed, affect what that process views
when it performs a subsequent filesystem access.
Volumes are mounted at