---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#2-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 71
summary: ## Container hooks There are two hooks that are exposed to Containers: `PostStart` This hook is executed immediately after a container is created. It runs **concurrently** with the container's...
---

## Container hooks
There are two hooks that are exposed to Containers:
`PostStart`
This hook is executed immediately after a container is created.
It runs **concurrently** with the container's `ENTRYPOINT` (main process),
meaning the hook may run before, during, or after the main process starts.
No parameters are passed to the handler.