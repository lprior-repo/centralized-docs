---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#113-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 121
summary: This meant that adding a new storage system to Kubernetes (a volume plugin) required checking code into the core Kubernetes code repository. Both CSI and FlexVolume allow volume plugins to be...
---

This meant that adding a new storage system to
Kubernetes (a volume plugin) required checking code into the core Kubernetes code repository.
Both CSI and FlexVolume allow volume plugins to be developed independently of
the Kubernetes code base, and deployed (installed) on Kubernetes clusters as
extensions.
For storage vendors looking to create an out-of-tree volume plugin, please refer
to the [volume plugin FAQ](https://github.com/kubernetes/community/blob/master/sig-storage/volume-plugin-faq.md).
### csi
[Container Storage Interface](https://github.com/container-storage-interface/spec/blob/master/spec.md)