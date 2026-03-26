---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: FEATURE STATE: `Kubernetes v1.20 [stable]` Kubernetes allow you to limit the number of process IDs (PIDs) that a [Pod](/docs/concepts/workloads/pods/) can use. You can also reserve a number of...
---

FEATURE STATE:
`Kubernetes v1.20 [stable]`
Kubernetes allow you to limit the number of process IDs (PIDs) that a
[Pod](/docs/concepts/workloads/pods/) can use.
You can also reserve a number of allocatable PIDs for each [node](/docs/concepts/architecture/nodes/)
for use by the operating system and daemons (rather than by Pods).
Process IDs (PIDs) are a fundamental resource on nodes. It is trivial to hit the
task limit without hitting any other resource limits, which can then cause
instability to a host machine.