---
doc_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging
chunk_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging#5-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 111
summary: ## Before you begin * Kubernetes cluster is installed * `kubectl` is configured to communicate with the cluster * [Telepresence](https://www.telepresence.io/docs/latest/quick-start/) is installed##...
---

## Before you begin
* Kubernetes cluster is installed
* `kubectl` is configured to communicate with the cluster
* [Telepresence](https://www.telepresence.io/docs/latest/quick-start/) is installed## Connecting your local machine to a remote Kubernetes cluster
After installing `telepresence`, run `telepresence connect` to launch
its Daemon and connect your local workstation to the cluster.
```
`$ telepresence connect
Launching Telepresence Daemon
...
Connected to context default (https://&lt;cluster public IP&gt;)
`
```