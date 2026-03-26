---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#16-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 123
summary: * Superuser privileges on all machines using `sudo` * You can use a different tool; this guide uses `sudo` in the examples. * SSH access from one device to all nodes in the system * `kubeadm` and...
---

* Superuser privileges on all machines using `sudo`
* You can use a different tool; this guide uses `sudo` in the examples.
* SSH access from one device to all nodes in the system
* `kubeadm` and `kubelet` already installed on all machines.
And you also need:
* Three or more additional machines, that will become etcd cluster members.
Having an odd number of members in the etcd cluster is a requirement for achieving
optimal voting quorum.
* These machines again need to have `kubeadm` and `kubelet` installed.