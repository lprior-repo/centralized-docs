---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Taint based Evictions
token_count: 809
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
#### Note:
The control plane limits the rate of adding new taints to nodes. This rate limiting
manages the number of evictions that are triggered when many nodes become unreachable at
once (for example: if there is a network disruption).
You can specify `tolerationSeconds` for a Pod to define how long that Pod stays bound
to a failing or unresponsive Node.
For example, you might want to keep an application with a lot of local state
bound to node for a long time in the event of network partition, hoping
that the partition will recover and thus the pod eviction can be avoided.
The toleration you set for that Pod might look like:
```
`tolerations:
- key: "node.kubernetes.io/unreachable"
operator: "Exists"
effect: "NoExecute"
tolerationSeconds: 6000
`
```
#### Note:
Kubernetes automatically adds a toleration for
`node.kubernetes.io/not-ready` and `node.kubernetes.io/unreachable`
with `tolerationSeconds=300`,
unless you, or a controller, set those tolerations explicitly.
These automatically-added tolerations mean that Pods remain bound to
Nodes for 5 minutes after one of these problems is detected.
[DaemonSet](/docs/concepts/workloads/controllers/daemonset/) pods are created with
`NoExecute` tolerations for the following taints with no `tolerationSeconds`:
* `node.kubernetes.io/unreachable`
* `node.kubernetes.io/not-ready`
This ensures that DaemonSet pods are never evicted due to these problems.
#### Note:
The node controller was responsible for adding taints to nodes and evicting pods. But after 1.29,
the taint-based eviction implementation has been moved out of node controller into a separate,
and independent component called taint-eviction-controller. Users can optionally disable taint-based
eviction by setting `--controllers=-taint-eviction-controller` in kube-controller-manager.