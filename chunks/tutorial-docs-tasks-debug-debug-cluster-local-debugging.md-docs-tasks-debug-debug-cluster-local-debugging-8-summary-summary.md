---
doc_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging
chunk_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging#8-summary
chunk_level: summary
chunk_type: prose
heading: Developing or debugging an existing service
token_count: 119
summary: * `$SERVICE\_NAME` is the name of your local service * `$LOCAL\_PORT` is the port that your service is running on your local workstation * And `$REMOTE\_PORT` is the port your service listens to in...
---

* `$SERVICE\_NAME` is the name of your local service
* `$LOCAL\_PORT` is the port that your service is running on your local workstation
* And `$REMOTE\_PORT` is the port your service listens to in the cluster
Running this command tells Telepresence to send remote traffic to your
local service instead of the service in the remote Kubernetes cluster.
Make edits to your service source code locally, save, and see the corresponding
changes when accessing your remote application take effect immediately.
You can also run your local service using a debugger or any other local development tool.