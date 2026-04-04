---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#110-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 51
summary: * Stops the kubelet. * Stops running containers. * Unmounts any mounted directories in `/var/lib/kubelet`. * Deletes any files and directories managed by kubeadm in `/var/lib/kubelet` and...
---

* Stops the kubelet.
* Stops running containers.
* Unmounts any mounted directories in `/var/lib/kubelet`.
* Deletes any files and directories managed by kubeadm in `/var/lib/kubelet` and `/etc/kubernetes`.