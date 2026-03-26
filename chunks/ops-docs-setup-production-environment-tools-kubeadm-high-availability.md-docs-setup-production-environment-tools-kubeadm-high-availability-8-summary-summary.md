---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 119
summary: * With an external etcd cluster. This approach requires more infrastructure. The control plane nodes and etcd members are separated. Before proceeding, you should carefully consider which approach...
---

* With an external etcd cluster. This approach requires more infrastructure. The
control plane nodes and etcd members are separated.
Before proceeding, you should carefully consider which approach best meets the needs of your applications
and environment. [Options for Highly Available topology](/docs/setup/production-environment/tools/kubeadm/ha-topology/)
outlines the advantages and disadvantages of each.
If you encounter issues with setting up the HA cluster, please report these
in the kubeadm [issue tracker](https://github.com/kubernetes/kubeadm/issues/new).
See also the