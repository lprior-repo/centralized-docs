---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#3-standard
chunk_level: standard
chunk_type: prose
heading: Benefits of a built-in sidecar container
token_count: 241
summary: ## Benefits of a built-in sidecar container Using Kubernetes' native support for sidecar containers provides several benefits: 1. You can configure a native sidecar container to start ahead of [init...
---

## Benefits of a built-in sidecar container
Using Kubernetes' native support for sidecar containers provides several benefits:
1. You can configure a native sidecar container to start ahead of
[init containers](/docs/concepts/workloads/pods/init-containers/).
2. The built-in sidecar containers can be authored to guarantee that they are terminated last.
Sidecar containers are terminated with a `SIGTERM` signal once all the regular containers
are completed and terminated. If the sidecar container isn’t gracefully shut down, a
`SIGKILL` signal will be used to terminate it.
3. With Jobs, when Pod's `restartPolicy: OnFailure` or `restartPolicy: Never`,
native sidecar containers do not block Pod completion. With legacy sidecar containers,
special care is needed to handle this situation.
4. Also, with Jobs, built-in sidecar containers would keep being restarted once they are done,
even if regular containers would not with Pod's `restartPolicy: Never`.
See [differences from init containers](/docs/concepts/workloads/pods/sidecar-containers/#differences-from-application-containers)
to learn more about it.