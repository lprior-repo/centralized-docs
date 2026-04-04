---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#31-standard
chunk_level: standard
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 512
summary: Kubernetes releases prior to v1.25 did not include this support. * Enable the [feature gate](/docs/reference/command-line-tools-reference/feature-gates-removed/) named `CSINodeExpandSecret` for each...
---

Kubernetes releases
prior to v1.25 did not include this support.
* Enable the [feature gate](/docs/reference/command-line-tools-reference/feature-gates-removed/)
named `CSINodeExpandSecret` for each kube-apiserver and for the kubelet on every
node. Since Kubernetes version 1.27, this feature has been enabled by default
and no explicit enablement of the feature gate is required.
You must also be using a CSI driver that supports or requires secret data during
node-initiated storage resize operations.
* `nodePublishSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`NodePublishVolume` call. This field is optional and may be empty if no
secret is required. If the secret object contains more than one secret, all
secrets are passed.
* `nodeStageSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`NodeStageVolume` call. This field is optional and may be empty if no secret
is required. If the Secret contains more than one secret, all secrets
are passed.#### CSI raw block volume support
FEATURE STATE:
`Kubernetes v1.18 [stable]`
Vendors with external CSI drivers can implement raw block volume support
in Kubernetes workloads.
You can set up your
[PersistentVolume/PersistentVolumeClaim with raw block volume support](/docs/concepts/storage/persistent-volumes/#raw-block-volume-support)
as usual, without any CSI-specific changes.
#### CSI ephemeral volumes
FEATURE STATE:
`Kubernetes v1.25 [stable]`
You can directly configure CSI volumes within the Pod
specification. Volumes specified in this way are ephemeral and do not
persist across Pod restarts. See
[Ephemeral Volumes](/docs/concepts/storage/ephemeral-volumes/#csi-ephemeral-volumes)
for more information.
For more information on how to develop a CSI driver, refer to the
[kubernetes-csi documentation](https://kubernetes-csi.github.io/docs/)
#### Windows CSI proxy
FEATURE STATE:
`Kubernetes v1.22 [stable]`
CSI node plugins need to perform various privileged
operations like scanning of disk devices and mounting of file systems. These operations
differ for each host operating system. For Linux worker nodes, containerized CSI node
plugins are typically deployed as privileged containers.