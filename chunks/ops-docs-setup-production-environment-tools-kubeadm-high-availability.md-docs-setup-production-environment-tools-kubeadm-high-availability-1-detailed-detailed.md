---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 244
summary: # Creating Highly Available Clusters with kubeadm This page explains two different approaches to setting up a highly available Kubernetes cluster using kubeadm: * With stacked control plane nodes....
---

# Creating Highly Available Clusters with kubeadm
This page explains two different approaches to setting up a highly available Kubernetes
cluster using kubeadm:
* With stacked control plane nodes. This approach requires less infrastructure. The etcd members
and control plane nodes are co-located.
* With an external etcd cluster. This approach requires more infrastructure. The
control plane nodes and etcd members are separated.
Before proceeding, you should carefully consider which approach best meets the needs of your applications
and environment. [Options for Highly Available topology](/docs/setup/production-environment/tools/kubeadm/ha-topology/)
outlines the advantages and disadvantages of each.
If you encounter issues with setting up the HA cluster, please report these
in the kubeadm [issue tracker](https://github.com/kubernetes/kubeadm/issues/new).
See also the [upgrade documentation](/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/).
#### Caution:
This page does not address running your cluster on a cloud provider. In a cloud
environment, neither approach documented here works with Service objects of type
LoadBalancer, or with dynamic PersistentVolumes.