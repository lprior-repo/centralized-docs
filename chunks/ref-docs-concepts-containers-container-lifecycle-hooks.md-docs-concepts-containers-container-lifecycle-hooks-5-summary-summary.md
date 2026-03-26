---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#5-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 128
summary: 's termination grace period countdown begins before the `PreStop` hook is executed, so regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination...
---

's termination
grace period countdown begins before the `PreStop` hook is executed, so regardless of the outcome of
the handler, the container will eventually terminate within the Pod's termination grace period. No
parameters are passed to the handler.
A more detailed description of the termination behavior can be found in
[Termination of Pods](/docs/concepts/workloads/pods/pod-lifecycle/#pod-termination).
`StopSignal`
The StopSignal lifecycle can be used to define a stop signal which would be sent to the container when it is
stopped. If you set this, it overrides any `STOPSIGNAL`