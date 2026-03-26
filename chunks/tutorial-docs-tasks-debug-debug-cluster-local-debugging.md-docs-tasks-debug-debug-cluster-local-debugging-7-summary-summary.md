---
doc_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging
chunk_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging#7-summary
chunk_level: summary
chunk_type: prose
heading: Developing or debugging an existing service
token_count: 105
summary: ## Developing or debugging an existing service When developing an application on Kubernetes, you typically program or debug a single service. The service might require access to other services for...
---

## Developing or debugging an existing service
When developing an application on Kubernetes, you typically program
or debug a single service. The service might require access to other
services for testing and debugging. One option is to use the continuous
deployment pipeline, but even the fastest deployment pipeline introduces
a delay in the program or debug cycle.
Use the `telepresence intercept $SERVICE\_NAME --port $LOCAL\_PORT:$REMOTE\_PORT`
command to create an "intercept" for rerouting remote service traffic.
Where: