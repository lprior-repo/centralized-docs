---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 124
summary: If the Pod is unhealthy due to a permanent fault, scaling without correcting the fault may lead to a state where the StatefulSet membership drops below a certain minimum number of replicas that are...
---

If the Pod is unhealthy due to a permanent fault, scaling
without correcting the fault may lead to a state where the StatefulSet membership
drops below a certain minimum number of replicas that are needed to function
correctly. This may cause your StatefulSet to become unavailable.
If the Pod is unhealthy due to a transient fault and the Pod might become available again,
the transient error may interfere with your scale-up or scale-down operation. Some distributed
databases have issues when nodes join and leave at the same time. It is better
to reason about scaling operations at the application level in these cases, and