---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#17-summary
chunk_level: summary
chunk_type: prose
heading: Benefits of a built-in sidecar container
token_count: 118
summary: 3. With Jobs, when Pod's `restartPolicy: OnFailure` or `restartPolicy: Never`, native sidecar containers do not block Pod completion. With legacy sidecar containers, special care is needed to handle...
---

3. With Jobs, when Pod's `restartPolicy: OnFailure` or `restartPolicy: Never`,
native sidecar containers do not block Pod completion. With legacy sidecar containers,
special care is needed to handle this situation.
4. Also, with Jobs, built-in sidecar containers would keep being restarted once they are done,
even if regular containers would not with Pod's `restartPolicy: Never`.
See [differences from init containers](/docs/concepts/workloads/pods/sidecar-containers/#differences-from-application-containers)
to learn more about it.