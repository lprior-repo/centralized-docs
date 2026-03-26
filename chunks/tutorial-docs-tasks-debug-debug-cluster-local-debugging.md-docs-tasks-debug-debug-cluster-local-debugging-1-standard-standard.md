---
doc_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging
chunk_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging#1-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 382
summary: # Developing and debugging services locally using telepresence **Note:** This section links to third party projects that provide functionality required by Kubernetes. The Kubernetes project authors...
---

# Developing and debugging services locally using telepresence
**Note:** This section links to third party projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for these projects, which are listed alphabetically. To add a project to this list, read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before submitting a change. [More information.](#third-party-content-disclaimer)
Kubernetes applications usually consist of multiple, separate services,
each running in its own container. Developing and debugging these services
on a remote Kubernetes cluster can be cumbersome, requiring you to
[get a shell on a running container](/docs/tasks/debug/debug-application/get-shell-running-container/)
in order to run debugging tools.
`telepresence` is a tool to ease the process of developing and debugging
services locally while proxying the service to a remote Kubernetes cluster.
Using `telepresence` allows you to use custom tools, such as a debugger and
IDE, for a local service and provides the service full access to ConfigMap,
secrets, and the services running on the remote cluster.
This document describes using `telepresence` to develop and debug services
running on a remote cluster locally.
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
You can curl services using the Kubernetes syntax e.g. `curl -ik https://kubernetes.default`