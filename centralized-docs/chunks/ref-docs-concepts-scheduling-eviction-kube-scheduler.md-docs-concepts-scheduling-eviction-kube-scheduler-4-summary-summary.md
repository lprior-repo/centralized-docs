---
doc_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler
chunk_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler#4-summary
chunk_level: summary
chunk_type: prose
heading: kube-scheduler
token_count: 127
summary: [kube-scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/) is the default scheduler for Kubernetes and runs as part of the [control...
---

[kube-scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/)
is the default scheduler for Kubernetes and runs as part of the
[control plane](/docs/reference/glossary/?all=true#term-control-plane).
kube-scheduler is designed so that, if you want and need to, you can
write your own scheduling component and use that instead.
Kube-scheduler selects an optimal node to run newly created or not yet
scheduled (unscheduled) pods. Since containers in pods - and pods themselves -
can have different requirements, the scheduler filters out any nodes that
don't meet a Pod'