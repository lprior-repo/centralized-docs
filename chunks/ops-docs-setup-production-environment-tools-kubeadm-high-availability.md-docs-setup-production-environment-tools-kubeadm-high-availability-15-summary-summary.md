---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#15-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 126
summary: * including a [container runtime](/docs/setup/production-environment/container-runtimes), already set up and working * Three or more machines that meet [kubeadm's minimum...
---

* including a [container runtime](/docs/setup/production-environment/container-runtimes), already set up and working
* Three or more machines that meet [kubeadm's minimum
requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for the workers
* including a container runtime, already set up and working
* Full network connectivity between all machines in the cluster (public or
private network)
* Superuser privileges on all machines using `sudo`
* You can use a different tool; this guide uses `sudo` in the examples.