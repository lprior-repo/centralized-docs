---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#138-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 122
summary: However, the CRI runtime may choose `rslave` mount propagation (i.e., `HostToContainer`) when `rprivate` propagation is not applicable. cri-dockerd (Docker) is known to choose `rslave` mount...
---

However, the CRI runtime may choose `rslave` mount propagation (i.e.,
`HostToContainer`) when `rprivate` propagation is not applicable.
cri-dockerd (Docker) is known to choose `rslave` mount propagation when the
mount source contains the Docker daemon's root directory (`/var/lib/docker`).
* `HostToContainer` - This volume mount will receive all subsequent mounts
that are mounted to this volume or any of its subdirectories.
In other words, if the host mounts anything inside the volume mount, the
container will see it mounted there.