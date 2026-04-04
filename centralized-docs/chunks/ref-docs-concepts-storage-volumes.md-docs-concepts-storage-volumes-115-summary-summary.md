---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#115-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 117
summary: v1.13 and will be removed in a future release. #### Note: CSI drivers may not be compatible across all Kubernetes releases. Please check the specific CSI driver's documentation for supported...
---

v1.13 and will be removed in a future release.
#### Note:
CSI drivers may not be compatible across all Kubernetes releases.
Please check the specific CSI driver's documentation for supported
deployment steps for each Kubernetes release and a compatibility matrix.
Once a CSI-compatible volume driver is deployed on a Kubernetes cluster, users
may use the `csi` volume type to attach or mount the volumes exposed by the
CSI driver.
A `csi` volume can be used in a Pod in three different ways:
* through a reference to a [PersistentVolumeClaim](#persistentvolumeclaim)