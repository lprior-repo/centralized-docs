---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Concepts
token_count: 913
summary: #### Note: There are two special cases: If the `key` is empty, then the `operator` must be `Exists`, which matches all keys and values. Note that the `effect` still needs to be matched at the same...
---

#### Note:
There are two special cases:
If the `key` is empty, then the `operator` must be `Exists`, which matches all keys and values.
Note that the `effect` still needs to be matched at the same time.
An empty `effect` matches all effects with key `key1`.
The above example used the `effect` of `NoSchedule`. Alternatively, you can use the `effect` of `PreferNoSchedule`.
The allowed values for the `effect` field are:
`NoExecute`This affects pods that are already running on the node as follows:
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
```
`tolerations:
- key: "key1"
operator: "Equal"
value: "value1"
effect: "NoSchedule"
- key: "key1"
operator: "Equal"
value: "value1"
effect: "NoExecute"
`
```
In this case, the pod will not be able to schedule onto the node, because there is no
toleration matching the third taint. But it will be able to continue running if it is
already running on the node when the taint is added, because the third taint is the only
one of the three that is not tolerated by the pod.
Normally, if a taint with effect `NoExecute` is added to a node, then any pods that do
not tolerate the taint will be evicted immediately, and pods that do tolerate the
taint will never be evicted. However, a toleration with `NoExecute` effect can specify
an optional `tolerationSeconds` field that dictates how long the pod will stay bound
to the node after the taint is added. For example,
```
`tolerations:
- key: "key1"
operator: "Equal"
value: "value1"
effect: "NoExecute"
tolerationSeconds: 3600
`
```
means that if this pod is running and a matching taint is added to the node, then
the pod will stay bound to the node for 3600 seconds, and then be evicted. If the
taint is removed before that time, the pod will not be evicted.