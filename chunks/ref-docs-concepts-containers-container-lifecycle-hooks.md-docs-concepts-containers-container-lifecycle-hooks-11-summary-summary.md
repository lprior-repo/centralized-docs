---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#11-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 128
summary: complete its execution before the TERM signal can be sent. If a `PreStop` hook hangs during execution, the Pod's phase will be `Terminating` and remain there until the Pod is killed after its...
---

complete its execution before the TERM signal can be sent. If a `PreStop` hook hangs during
execution, the Pod's phase will be `Terminating` and remain there until the Pod is killed after its
`terminationGracePeriodSeconds` expires. This grace period applies to the total time it takes for
both the `PreStop` hook to execute and for the Container to stop normally. If, for example,
`terminationGracePeriodSeconds` is 60, and the hook takes 55 seconds to complete, and the Container
takes 10 seconds to stop normally after receiving the signal, then the Container will be killed