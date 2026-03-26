---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#10-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 82
summary: * **You are using `hostPort`**: When you bind a Pod to a `hostPort` there are a limited number of places that pod can be scheduled. In most cases, `hostPort` is unnecessary, try using a Service...
---

* **You are using `hostPort`**: When you bind a Pod to a `hostPort` there are a
limited number of places that pod can be scheduled. In most cases, `hostPort`
is unnecessary, try using a Service object to expose your Pod. If you do require
`hostPort` then you can only schedule as many Pods as there are nodes in your Kubernetes cluster.