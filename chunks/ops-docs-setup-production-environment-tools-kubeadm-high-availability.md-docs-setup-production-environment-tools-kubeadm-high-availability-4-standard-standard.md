---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#4-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 276
summary: * including a container runtime, already set up and working * Full network connectivity between all machines in the cluster (public or private network) * Superuser privileges on all machines using...
---

* including a container runtime, already set up and working
* Full network connectivity between all machines in the cluster (public or
private network)
* Superuser privileges on all machines using `sudo`
* You can use a different tool; this guide uses `sudo` in the examples.
* SSH access from one device to all nodes in the system
* `kubeadm` and `kubelet` already installed on all machines.
And you also need:
* Three or more additional machines, that will become etcd cluster members.
Having an odd number of members in the etcd cluster is a requirement for achieving
optimal voting quorum.
* These machines again need to have `kubeadm` and `kubelet` installed.
* These machines also require a container runtime, that is already set up and working.
*See [External etcd topology](/docs/setup/production-environment/tools/kubeadm/ha-topology/#external-etcd-topology) for context.*
### Container images
Each host should have access read and fetch images from the Kubernetes container image registry,
`registry.k8s.io`. If you want to deploy a highly-available cluster where the hosts do not have
access to pull images, this is possible. You must ensure by some other means that the correct
container images are already available on the relevant hosts.