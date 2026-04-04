---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#17-summary
chunk_level: summary
chunk_type: prose
heading: Observability
token_count: 75
summary: ## Observability The metric `scheduler\_pending\_pods` comes with a new label `\"gated\"` to distinguish whether a Pod has been tried scheduling but claimed as unschedulable, or explicitly marked as...
---

## Observability
The metric `scheduler\_pending\_pods` comes with a new label `"gated"` to distinguish whether a Pod
has been tried scheduling but claimed as unschedulable, or explicitly marked as not ready for
scheduling. You can use `scheduler\_pending\_pods{queue="gated"}` to check the metric result.