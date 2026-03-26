---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#9-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 128
summary: * Sleep - Pauses the container for a specified duration.### Hook handler execution When a Container lifecycle management hook is called, the Kubernetes management system executes the handler...
---

* Sleep - Pauses the container for a specified duration.### Hook handler execution
When a Container lifecycle management hook is called,
the Kubernetes management system executes the handler according to the hook action,
`httpGet`, `tcpSocket` ([deprecated](/docs/reference/generated/kubernetes-api/v1.35/#lifecyclehandler-v1-core))
and `sleep` are executed by the kubelet process, and `exec` is executed in the container.
The `PostStart` hook handler call is initiated when a container is created,
meaning the container ENTRYPOINT and the `PostStart` hook are triggered simultaneously.
(This means it generally doesn'