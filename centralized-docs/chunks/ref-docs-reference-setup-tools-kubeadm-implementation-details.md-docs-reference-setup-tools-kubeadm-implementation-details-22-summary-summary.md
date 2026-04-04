---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#22-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 116
summary: * [Warning] if the Kubernetes version to use (specified with the `--kubernetes-version` flag) is at least one minor version higher than the kubeadm CLI version. * Kubernetes system requirements: * if...
---

* [Warning] if the Kubernetes version to use (specified with the `--kubernetes-version` flag) is
at least one minor version higher than the kubeadm CLI version.
* Kubernetes system requirements:
* if running on linux:
* [Error] if Kernel is older than the minimum required version
* [Error] if required cgroups subsystem aren't set up
* [Error] if the CRI endpoint does not answer
* [Error] if user is not root
* [Error] if the machine hostname is not a valid DNS subdomain