---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Why should I use `ReadWriteOncePod`?
token_count: 593
summary: # Change the Access Mode of a PersistentVolume to ReadWriteOncePod This page shows how to change the access mode on an existing PersistentVolume to use `ReadWriteOncePod`. ## Before you begin You...
---

# Change the Access Mode of a PersistentVolume to ReadWriteOncePod
This page shows how to change the access mode on an existing PersistentVolume to
use `ReadWriteOncePod`.
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)Your Kubernetes server must be at or later than version v1.22.
To check the version, enter `kubectl version`.
#### Note:
The `ReadWriteOncePod` access mode graduated to stable in the Kubernetes v1.29
release. If you are running a version of Kubernetes older than v1.29, you might
need to enable a feature gate. Check the documentation for your version of
Kubernetes.
#### Note:
The `ReadWriteOncePod` access mode is only supported for
[CSI](/docs/concepts/storage/volumes/#csi) volumes.
To use this volume access mode you will need to update the following
[CSI sidecars](https://kubernetes-csi.github.io/docs/sidecar-containers.html)
to these versions or greater:
* [csi-provisioner:v3.0.0+](https://github.com/kubernetes-csi/external-provisioner/releases/tag/v3.0.0)
* [csi-attacher:v3.3.0+](https://github.com/kubernetes-csi/external-attacher/releases/tag/v3.3.0)
* [csi-resizer:v1.3.0+](https://github.com/kubernetes-csi/external-resizer/releases/tag/v1.3.0)
## Why should I use `ReadWriteOncePod`?
Prior to Kubernetes v1.22, the `ReadWriteOnce` access mode was commonly used to
restrict PersistentVolume access for workloads that required single-writer
access to storage. However, this access mode had a limitation: it restricted
volume access to a single *node*, allowing multiple pods on the same node to
read from and write to the same volume simultaneously. This could pose a risk
for applications that demand strict single-writer access for data safety.
If ensuring single-writer access is critical for your workloads, consider
migrating your volumes to `ReadWriteOncePod`.