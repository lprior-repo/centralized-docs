---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#49-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 76
summary: * Replace the following variables in the config template with the appropriate values for your cluster: * `LOAD\_BALANCER\_DNS` * `LOAD\_BALANCER\_PORT` * `ETCD\_0\_IP` * `ETCD\_1\_IP` * `ETCD\_2\_IP`...
---

* Replace the following variables in the config template with the appropriate values for your cluster:
* `LOAD\_BALANCER\_DNS`
* `LOAD\_BALANCER\_PORT`
* `ETCD\_0\_IP`
* `ETCD\_1\_IP`
* `ETCD\_2\_IP`
The following steps are similar to the stacked etcd setup: