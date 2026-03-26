---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#16-summary
chunk_level: summary
chunk_type: prose
heading: Benefits of a built-in sidecar container
token_count: 100
summary: 1. You can configure a native sidecar container to start ahead of [init containers](/docs/concepts/workloads/pods/init-containers/). 2. The built-in sidecar containers can be authored to guarantee...
---

1. You can configure a native sidecar container to start ahead of
[init containers](/docs/concepts/workloads/pods/init-containers/).
2. The built-in sidecar containers can be authored to guarantee that they are terminated last.
Sidecar containers are terminated with a `SIGTERM` signal once all the regular containers
are completed and terminated. If the sidecar container isn’t gracefully shut down, a
`SIGKILL` signal will be used to terminate it.