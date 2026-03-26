---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#2-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 119
summary: ## Before you begin * StatefulSets are only available in Kubernetes version 1.5 or later. To check your version of Kubernetes, run `kubectl version`. * Not all stateful applications scale nicely. If...
---

## Before you begin
* StatefulSets are only available in Kubernetes version 1.5 or later.
To check your version of Kubernetes, run `kubectl version`.
* Not all stateful applications scale nicely. If you are unsure about whether
to scale your StatefulSets, see [StatefulSet concepts](/docs/concepts/workloads/controllers/statefulset/)
or [StatefulSet tutorial](/docs/tutorials/stateful-application/basic-stateful-set/) for further information.
* You should perform scaling only when you are confident that your stateful application
cluster is completely healthy.