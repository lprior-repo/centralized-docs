---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#94-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 115
summary: ### persistentVolumeClaim A `persistentVolumeClaim` volume is used to mount a [PersistentVolume](/docs/concepts/storage/persistent-volumes/) into a Pod. PersistentVolumeClaims are a way for users to...
---

### persistentVolumeClaim
A `persistentVolumeClaim` volume is used to mount a
[PersistentVolume](/docs/concepts/storage/persistent-volumes/) into a Pod. PersistentVolumeClaims
are a way for users to "claim" durable storage (such as an iSCSI volume)
without knowing the details of the particular cloud environment.
See the information about [PersistentVolumes](/docs/concepts/storage/persistent-volumes/) for more
details.
### portworxVolume (deprecated)
FEATURE STATE:
`Kubernetes v1.25 [deprecated]`