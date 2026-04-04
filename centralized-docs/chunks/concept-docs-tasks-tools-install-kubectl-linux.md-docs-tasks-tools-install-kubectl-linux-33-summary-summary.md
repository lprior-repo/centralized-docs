---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#33-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 98
summary: For example, if you are intending to run a Kubernetes cluster on your laptop (locally), you will need a tool like [Minikube](https://minikube.sigs.k8s.io/docs/start/) to be installed first and then...
---

For example, if you are intending to run a Kubernetes cluster on your laptop (locally),
you will need a tool like [Minikube](https://minikube.sigs.k8s.io/docs/start/) to be
installed first and then re-run the commands stated above.
If `kubectl cluster-info` returns the url response, but you can't access your cluster,
check whether it is configured properly using the following command:
```
`kubectl cluster-info dump
`
```