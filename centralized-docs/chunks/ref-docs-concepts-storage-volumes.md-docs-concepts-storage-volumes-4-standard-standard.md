---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#4-standard
chunk_level: standard
chunk_type: prose
heading: Why volumes are important
token_count: 449
summary: # Volumes Kubernetes *volumes* provide a way for containers in a [Pod](/docs/concepts/workloads/pods/) to access and share data via the filesystem. There are different kinds of volume that you can...
---

# Volumes
Kubernetes *volumes* provide a way for containers in a [Pod](/docs/concepts/workloads/pods/)
to access and share data via the filesystem. There are different kinds of volume that you can use for different purposes,
such as:
* populating a configuration file based on a [ConfigMap](/docs/concepts/configuration/configmap/)
or a [Secret](/docs/concepts/configuration/secret/)
* providing some temporary scratch space for a Pod
* sharing a filesystem between two different containers in the same Pod
* sharing a filesystem between two different Pods (even if those Pods run on different nodes)
* durably storing data so that it stays available even if the Pod restarts or is replaced
* passing configuration information to an app running in a container, based on details of the Pod
the container is in
(for example: telling a [sidecar container](/docs/concepts/workloads/pods/sidecar-containers/)
what namespace the Pod is running in)
* providing read-only access to data in a different container image
Data sharing can be between different local processes within a container, or between different containers,
or between Pods.
## Why volumes are important
* **Data persistence:** On-disk files in a container are ephemeral, which presents some problems for
non-trivial applications when running in containers. One problem occurs when
a container crashes or is stopped; the container state is not saved, so all of the
files that were created or modified during the lifetime of the container are lost.
After a crash, kubelet restarts the container with a clean state.
* **Shared storage:** Another problem occurs when multiple containers are running in a `Pod` and
need to share files. It can be challenging to set up
and access a shared filesystem across all of the containers.
The Kubernetes [volume](/docs/concepts/storage/volumes/) abstraction
can help you to solve both of these problems.
Before you learn about volumes, PersistentVolumes, and PersistentVolumeClaims, you should read up
about [Pods](/docs/concepts/workloads/pods/) and make sure that you understand how
Kubernetes uses Pods to run containers.