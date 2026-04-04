---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#20-summary
chunk_level: summary
chunk_type: prose
heading: Disallow labeling nodes
token_count: 102
summary: If you use this KubeSchedulerConfiguration, and don't run any custom scheduler, and you then define a Pod with `.spec.schedulerName` set to `nonexistent-scheduler` (or any other scheduler name that...
---

If you use this KubeSchedulerConfiguration, and don't run any custom scheduler,
and you then define a Pod with `.spec.schedulerName` set to `nonexistent-scheduler`
(or any other scheduler name that doesn't exist in your cluster), no events would be generated for a pod.
## Disallow labeling nodes
A cluster administrator should ensure that cluster users cannot label the nodes.
A malicious actor can use `nodeSelector` to schedule workloads on nodes where those workloads should not be present.