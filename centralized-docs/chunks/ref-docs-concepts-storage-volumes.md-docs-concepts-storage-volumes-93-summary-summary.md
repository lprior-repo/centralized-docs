---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#93-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 110
summary: #### Note: You must have your own NFS server running with the share exported before you can use it. Also note that you can't specify NFS mount options in a Pod spec. You can either set mount options...
---

#### Note:
You must have your own NFS server running with the share exported before you can use it.
Also note that you can't specify NFS mount options in a Pod spec. You can either set mount options server-side or
use [/etc/nfsmount.conf](https://man7.org/linux/man-pages/man5/nfsmount.conf.5.html).
You can also mount NFS volumes via PersistentVolumes, which do allow you to set mount options.
### persistentVolumeClaim
A `persistentVolumeClaim` volume is used to mount a