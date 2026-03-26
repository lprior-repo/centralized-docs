---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#7-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 116
summary: You cannot scale down a StatefulSet when any of the stateful Pods it manages is unhealthy. Scaling down only takes place after those stateful Pods become running and ready. If spec.replicas &gt; 1,...
---

You cannot scale down a StatefulSet when any of the stateful Pods it manages is
unhealthy. Scaling down only takes place after those stateful Pods become running and ready.
If spec.replicas &gt; 1, Kubernetes cannot determine the reason for an unhealthy Pod.
It might be the result of a permanent fault or of a transient fault. A transient
fault can be caused by a restart required by upgrading or maintenance.
If the Pod is unhealthy due to a permanent fault, scaling
without correcting the fault may lead to a state where the StatefulSet membership