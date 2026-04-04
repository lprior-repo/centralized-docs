---
doc_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler
chunk_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler#6-summary
chunk_level: summary
chunk_type: prose
heading: kube-scheduler
token_count: 105
summary: The scheduler finds feasible Nodes for a Pod and then runs a set of functions to score the feasible Nodes and picks a Node with the highest score among the feasible ones to run the Pod. The scheduler...
---

The scheduler finds feasible Nodes for a Pod and then runs a set of
functions to score the feasible Nodes and picks a Node with the highest
score among the feasible ones to run the Pod. The scheduler then notifies
the API server about this decision in a process called *binding*.
Factors that need to be taken into account for scheduling decisions include
individual and collective resource requirements, hardware / software /
policy constraints, affinity and anti-affinity specifications, data
locality, inter-workload interference, and so on.