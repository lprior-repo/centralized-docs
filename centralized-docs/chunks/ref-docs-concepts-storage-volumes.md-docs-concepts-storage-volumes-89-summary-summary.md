---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#89-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 127
summary: Note that this provisioner does not support dynamic provisioning yet. For an example on how to run an external local provisioner, see the [local volume provisioner user...
---

Note that this provisioner does not support dynamic
provisioning yet. For an example on how to run an external local provisioner, see the
[local volume provisioner user guide](https://github.com/kubernetes-sigs/sig-storage-local-static-provisioner).
#### Note:
The local PersistentVolume requires manual cleanup and deletion by the
user if the external static provisioner is not used to manage the volume
lifecycle.
### nfs
An `nfs` volume allows an existing NFS (Network File System) share to be
mounted into a Pod. Unlike `emptyDir`, which is erased when a Pod is