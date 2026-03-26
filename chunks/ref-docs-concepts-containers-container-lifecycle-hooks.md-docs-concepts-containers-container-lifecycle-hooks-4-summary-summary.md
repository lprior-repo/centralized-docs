---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#4-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 128
summary: While the hook runs concurrently with the container process, it can delay container status updates; the container may not transition to `Running` until the hook completes. `PreStop` This hook is...
---

While the hook runs concurrently with the container process,
it can delay container status updates;
the container may not transition to `Running` until the hook completes.
`PreStop`
This hook is called immediately before a container is terminated due to an API request or management
event such as a liveness/startup probe failure, preemption, resource contention and others. A call
to the `PreStop` hook fails if the container is already in a terminated or completed state and the
hook must complete before the TERM signal to stop the container can be sent. The Pod's termination
grace period countdown begins before the `PreStop`