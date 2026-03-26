---
doc_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging
chunk_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging#10-summary
chunk_level: summary
chunk_type: prose
heading: How does Telepresence work?
token_count: 128
summary: Telepresence installs a traffic-agent sidecar next to your existing application's container running in the remote cluster. It then captures all traffic requests going into the Pod, and instead of...
---

Telepresence installs a traffic-agent sidecar next to your existing
application's container running in the remote cluster. It then captures
all traffic requests going into the Pod, and instead of forwarding this
to the application in the remote cluster, it routes all traffic (when you
create a [global intercept](https://www.getambassador.io/docs/telepresence/latest/concepts/intercepts/#global-intercept)
or a subset of the traffic (when you create a
[personal intercept](https://www.getambassador.io/docs/telepresence/latest/concepts/intercepts/#personal-intercept))
to your local development environment.