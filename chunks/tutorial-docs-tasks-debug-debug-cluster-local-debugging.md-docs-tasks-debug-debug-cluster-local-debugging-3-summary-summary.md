---
doc_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging
chunk_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: each running in its own container. Developing and debugging these services on a remote Kubernetes cluster can be cumbersome, requiring you to [get a shell on a running...
---

each running in its own container. Developing and debugging these services
on a remote Kubernetes cluster can be cumbersome, requiring you to
[get a shell on a running container](/docs/tasks/debug/debug-application/get-shell-running-container/)
in order to run debugging tools.
`telepresence` is a tool to ease the process of developing and debugging
services locally while proxying the service to a remote Kubernetes cluster.
Using `telepresence` allows you to use custom tools, such as a debugger and
IDE, for a local service and provides the service full access to ConfigMap,
secrets, and the services running on the remote cluster.