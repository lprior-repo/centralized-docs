---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#48-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 57
summary: #### Note: The difference between stacked etcd and external etcd here is that the external etcd setup requires a configuration file with the etcd endpoints under the `external` object for `etcd`. In...
---

#### Note:
The difference between stacked etcd and external etcd here is that the external etcd setup requires
a configuration file with the etcd endpoints under the `external` object for `etcd`.
In the case of the stacked etcd topology, this is managed automatically.