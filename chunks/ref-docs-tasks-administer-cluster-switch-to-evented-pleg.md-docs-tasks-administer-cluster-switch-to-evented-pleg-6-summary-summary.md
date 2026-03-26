---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#6-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 62
summary: * This overhead is exacerbated by Kubelet's parallelized polling of container states, thus limiting its scalability and causing poor performance and reliability problems. * The goal of *Evented PLEG*...
---

* This overhead is exacerbated by Kubelet's parallelized polling of container states, thus limiting
its scalability and causing poor performance and reliability problems.
* The goal of *Evented PLEG* is to reduce unnecessary work during inactivity
by replacing periodic polling.## Switching to Evented PLEG