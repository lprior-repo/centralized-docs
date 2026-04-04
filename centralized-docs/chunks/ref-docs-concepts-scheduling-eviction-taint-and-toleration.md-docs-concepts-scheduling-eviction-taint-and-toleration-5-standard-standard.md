---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#5-standard
chunk_level: standard
chunk_type: prose
heading: Concepts
token_count: 464
summary: * Pods that do not tolerate the taint are evicted immediately * Pods that tolerate the taint without specifying `tolerationSeconds` in their toleration specification remain bound forever * Pods that...
---

* Pods that do not tolerate the taint are evicted immediately
* Pods that tolerate the taint without specifying `tolerationSeconds` in
their toleration specification remain bound forever
* Pods that tolerate the taint with a specified `tolerationSeconds` remain
bound for the specified amount of time. After that time elapses, the node
lifecycle controller evicts the Pods from the node.`NoSchedule`No new Pods will be scheduled on the tainted node unless they have a matching
toleration. Pods currently running on the node are **not** evicted.`PreferNoSchedule``PreferNoSchedule` is a "preference" or "soft" version of `NoSchedule`.
The control plane will *try* to avoid placing a Pod that does not tolerate
the taint on the node, but it is not guaranteed.
You can put multiple taints on the same node and multiple tolerations on the same pod.
The way Kubernetes processes multiple taints and tolerations is like a filter: start
with all of a node's taints, then ignore the ones for which the pod has a matching toleration; the
remaining un-ignored taints have the indicated effects on the pod. In particular,
* if there is at least one un-ignored taint with effect `NoSchedule` then Kubernetes will not schedule
the pod onto that node
* if there is no un-ignored taint with effect `NoSchedule` but there is at least one un-ignored taint with
effect `PreferNoSchedule` then Kubernetes will *try* to not schedule the pod onto the node
* if there is at least one un-ignored taint with effect `NoExecute` then the pod will be evicted from
the node (if it is already running on the node), and will not be
scheduled onto the node (if it is not yet running on the node).
For example, imagine you taint a node like this
```
`kubectl taint nodes node1 key1=value1:NoSchedule
kubectl taint nodes node1 key1=value1:NoExecute
kubectl taint nodes node1 key2=value2:NoSchedule
`
```
And a pod has two tolerations: