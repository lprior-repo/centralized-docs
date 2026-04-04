---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#12-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 115
summary: * The Kubernetes project recommends that you match your kubelet and kubeadm versions. You can instead use a version of kubelet that is older than kubeadm, provided it is within the range of supported...
---

* The Kubernetes project recommends that you match your kubelet and kubeadm versions.
You can instead use a version of kubelet that is older than kubeadm, provided it is within the
range of supported versions.
For more details, please visit [kubeadm's skew against the kubelet](/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#kubeadm-s-skew-against-the-kubelet).
* All containers are restarted after upgrade, because the container spec hash value is changed.