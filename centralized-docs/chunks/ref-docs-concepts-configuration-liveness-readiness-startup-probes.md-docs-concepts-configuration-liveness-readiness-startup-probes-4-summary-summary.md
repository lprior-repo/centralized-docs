---
doc_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes
chunk_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes#4-summary
chunk_level: summary
chunk_type: prose
heading: Readiness probe
token_count: 128
summary: `initialDelaySeconds` or use a [startup probe](#startup-probe). ## Readiness probe Readiness probes determine when a container is ready to accept traffic. This is useful when waiting for an...
---

`initialDelaySeconds` or use a
[startup probe](#startup-probe).
## Readiness probe
Readiness probes determine when a container is ready to accept traffic. This is useful when waiting for an application to perform time-consuming initial tasks that depend on its backing services; for example: establishing network connections, loading files, and warming caches. Readiness probes can also be useful later in the container’s lifecycle, for example, when recovering from temporary faults or overloads.
If the readiness probe returns a failed state, Kubernetes removes the pod from all matching service endpoints.
Readiness probes run on the container during its whole lifecycle.