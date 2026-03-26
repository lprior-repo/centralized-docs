---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#15-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 69
summary: * Make sure you are using the latest version. * Disable the webhook for `UPDATE` operations. * Report an issue with the corresponding provider. If you are the author of the webhook: * For a mutating...
---

* Make sure you are using the latest version.
* Disable the webhook for `UPDATE` operations.
* Report an issue with the corresponding provider.
If you are the author of the webhook:
* For a mutating webhook, make sure it never changes immutable fields on
`UPDATE` operations. For example, changes to containers are usually not allowed.