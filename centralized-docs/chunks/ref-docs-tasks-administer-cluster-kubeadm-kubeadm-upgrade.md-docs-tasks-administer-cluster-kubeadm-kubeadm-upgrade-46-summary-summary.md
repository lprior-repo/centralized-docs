---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#46-summary
chunk_level: summary
chunk_type: prose
heading: How it works
token_count: 124
summary: * Checks that your cluster is in an upgradeable state: * The API server is reachable * All nodes are in the `Ready` state * The control plane is healthy * Enforces the version skew policies. * Makes...
---

* Checks that your cluster is in an upgradeable state:
* The API server is reachable
* All nodes are in the `Ready` state
* The control plane is healthy
* Enforces the version skew policies.
* Makes sure the control plane images are available or available to pull to the machine.
* Generates replacements and/or uses user supplied overwrites if component configs require version upgrades.
* Upgrades the control plane components or rollbacks if any of them fails to come up.
* Applies the new `CoreDNS` and `kube-proxy` manifests and makes sure that all necessary RBAC rules are created.