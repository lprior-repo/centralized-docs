---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#13-standard
chunk_level: standard
chunk_type: prose
heading: Taint based Evictions
token_count: 401
summary: ## Taint based Evictions FEATURE STATE: `Kubernetes v1.18 [stable]` The node controller automatically taints a Node when certain conditions are true. The following taints are built in: *...
---

## Taint based Evictions
FEATURE STATE:
`Kubernetes v1.18 [stable]`
The node controller automatically taints a Node when certain conditions
are true. The following taints are built in:
* `node.kubernetes.io/not-ready`: Node is not ready. This corresponds to
the NodeCondition `Ready` being "`False`".
* `node.kubernetes.io/unreachable`: Node is unreachable from the node
controller. This corresponds to the NodeCondition `Ready` being "`Unknown`".
* `node.kubernetes.io/memory-pressure`: Node has memory pressure.
* `node.kubernetes.io/disk-pressure`: Node has disk pressure.
* `node.kubernetes.io/pid-pressure`: Node has PID pressure.
* `node.kubernetes.io/network-unavailable`: Node's network is unavailable.
* `node.kubernetes.io/unschedulable`: Node is unschedulable.
* `node.cloudprovider.kubernetes.io/uninitialized`: When the kubelet is started
with an "external" cloud provider, this taint is set on a node to mark it
as unusable. After a controller from the cloud-controller-manager initializes
this node, the kubelet removes this taint.
In case a node is to be drained, the node controller or the kubelet adds relevant taints
with `NoExecute` effect. This effect is added by default for the
`node.kubernetes.io/not-ready` and `node.kubernetes.io/unreachable` taints.
If the fault condition returns to normal, the kubelet or node
controller can remove the relevant taint(s).
In some cases when the node is unreachable, the API server is unable to communicate
with the kubelet on the node. The decision to delete the pods cannot be communicated to
the kubelet until communication with the API server is re-established. In the meantime,
the pods that are scheduled for deletion may continue to run on the partitioned node.