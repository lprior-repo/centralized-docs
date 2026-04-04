---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#18-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 120
summary: ## Verify kubectl configuration In order for kubectl to find and access a Kubernetes cluster, it needs a [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/), which is...
---

## Verify kubectl configuration
In order for kubectl to find and access a Kubernetes cluster, it needs a
[kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/),
which is created automatically when you create a cluster using
[kube-up.sh](https://github.com/kubernetes/kubernetes/blob/master/cluster/kube-up.sh)
or successfully deploy a Minikube cluster.
By default, kubectl configuration is located at `\~/.kube/config`.
Check that kubectl is properly configured by getting the cluster state:
```
`kubectl cluster-info
`
```