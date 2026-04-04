---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 572
summary: ## How it works `kubeadm upgrade apply` does the following: * Checks that your cluster is in an upgradeable state: * The API server is reachable * All nodes are in the `Ready` state * The control...
---

## How it works
`kubeadm upgrade apply` does the following:
* Checks that your cluster is in an upgradeable state:
* The API server is reachable
* All nodes are in the `Ready` state
* The control plane is healthy
* Enforces the version skew policies.
* Makes sure the control plane images are available or available to pull to the machine.
* Generates replacements and/or uses user supplied overwrites if component configs require version upgrades.
* Upgrades the control plane components or rollbacks if any of them fails to come up.
* Applies the new `CoreDNS` and `kube-proxy` manifests and makes sure that all necessary RBAC rules are created.
* Creates new certificate and key files of the API server and backs up old files if they're about to expire in 180 days.
`kubeadm upgrade node` does the following on additional control plane nodes:
* Fetches the kubeadm `ClusterConfiguration` from the cluster.
* Optionally backups the kube-apiserver certificate.
* Upgrades the static Pod manifests for the control plane components.
* Upgrades the kubelet configuration for this node.
`kubeadm upgrade node` does the following on worker nodes:
* Fetches the kubeadm `ClusterConfiguration` from the cluster.
* Upgrades the kubelet configuration for this node.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified December 08, 2025 at 6:53 PM PST: [Revise notes about cgroup v1 deprecation (b34a5979fd)](https://github.com/kubernetes/website/commit/b34a5979fd9dcee89bb2758cf5926c655c4d4403)
## Related Pages

- [Implementation details](docs-reference-setup-tools-kubeadm-implementation-details.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [and then append (or prepend) \~/.local/bin to $PATH](docs-tasks-tools-install-kubectl-linux.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)