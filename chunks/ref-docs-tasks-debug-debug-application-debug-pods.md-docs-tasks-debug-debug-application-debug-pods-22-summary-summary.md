---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#22-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 58
summary: . There will typically be some lines on the \"apiserver\" version that are not on the original version. This is expected. However, if there are lines on the original that are not on the apiserver...
---

. There will typically be some
lines on the "apiserver" version that are not on the original version. This is
expected. However, if there are lines on the original that are not on the apiserver
version, then this may indicate a problem with your pod spec.