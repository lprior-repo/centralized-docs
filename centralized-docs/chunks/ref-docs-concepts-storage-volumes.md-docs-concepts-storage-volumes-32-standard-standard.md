---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#32-standard
chunk_level: standard
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 511
summary: For more information on how to develop a CSI driver, refer to the [kubernetes-csi documentation](https://kubernetes-csi.github.io/docs/) #### Windows CSI proxy FEATURE STATE: `Kubernetes v1.22...
---

For more information on how to develop a CSI driver, refer to the
[kubernetes-csi documentation](https://kubernetes-csi.github.io/docs/)
#### Windows CSI proxy
FEATURE STATE:
`Kubernetes v1.22 [stable]`
CSI node plugins need to perform various privileged
operations like scanning of disk devices and mounting of file systems. These operations
differ for each host operating system. For Linux worker nodes, containerized CSI node
plugins are typically deployed as privileged containers. For Windows worker nodes,
privileged operations for containerized CSI node plugins are supported using
[csi-proxy](https://github.com/kubernetes-csi/csi-proxy), a community-managed,
stand-alone binary that needs to be pre-installed on each Windows node.
For more details, refer to the deployment guide of the CSI plugin you wish to deploy.
#### Migrating to CSI drivers from in-tree plugins
FEATURE STATE:
`Kubernetes v1.25 [stable]`
The `CSIMigration` feature directs operations against existing in-tree
plugins to corresponding CSI plugins (which are expected to be installed and configured).
As a result, operators do not have to make any
configuration changes to existing Storage Classes, PersistentVolumes, or PersistentVolumeClaims
(referring to in-tree plugins) when transitioning to a CSI driver that supersedes an in-tree plugin.
#### Note:
Existing PVs created by an in-tree volume plugin can still be used in the future without any configuration
changes, even after the migration to CSI is completed for that volume type, and even after you upgrade to a
version of Kubernetes that doesn't have compiled-in support for that kind of storage.
As part of that migration, you - or another cluster administrator - **must** have installed and configured
the appropriate CSI driver for that storage. The core of Kubernetes does not install that software for you.
After that migration, you can also define new PVCs and PVs that refer to the legacy, built-in
storage integrations.
Provided you have the appropriate CSI driver installed and configured, the PV creation continues
to work, even for brand-new volumes. The actual storage management now happens through
the CSI driver.
The operations and features that are supported include:
provisioning/delete, attach/detach, mount/unmount, and resizing of volumes.
In-tree plugins that support `CSIMigration` and have a corresponding CSI driver implemented
are listed in [Types of Volumes](#volume-types).
### flexVolume (deprecated)
FEATURE STATE: