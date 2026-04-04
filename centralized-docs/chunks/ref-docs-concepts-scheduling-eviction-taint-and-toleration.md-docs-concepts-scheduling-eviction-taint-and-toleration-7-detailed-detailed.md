---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#7-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 926
summary: ## Taint Nodes by Condition The control plane, using the node [controller](/docs/concepts/architecture/controller/), automatically creates taints with a `NoSchedule` effect for [node...
---

## Taint Nodes by Condition
The control plane, using the node [controller](/docs/concepts/architecture/controller/),
automatically creates taints with a `NoSchedule` effect for
[node conditions](/docs/concepts/scheduling-eviction/node-pressure-eviction/#node-conditions).
The scheduler checks taints, not node conditions, when it makes scheduling
decisions. This ensures that node conditions don't directly affect scheduling.
For example, if the `DiskPressure` node condition is active, the control plane
adds the `node.kubernetes.io/disk-pressure` taint and does not schedule new pods
onto the affected node. If the `MemoryPressure` node condition is active, the
control plane adds the `node.kubernetes.io/memory-pressure` taint.
You can ignore node conditions for newly created pods by adding the corresponding
Pod tolerations. The control plane also adds the `node.kubernetes.io/memory-pressure`
toleration on pods that have a [QoS class](/docs/concepts/workloads/pods/pod-qos/)
other than `BestEffort`. This is because Kubernetes treats pods in the `Guaranteed`
or `Burstable` QoS classes (even pods with no memory request set) as if they are
able to cope with memory pressure, while new `BestEffort` pods are not scheduled
onto the affected node.
The DaemonSet controller automatically adds the following `NoSchedule`
tolerations to all daemons, to prevent DaemonSets from breaking.
* `node.kubernetes.io/memory-pressure`
* `node.kubernetes.io/disk-pressure`
* `node.kubernetes.io/pid-pressure` (1.14 or later)
* `node.kubernetes.io/unschedulable` (1.10 or later)
* `node.kubernetes.io/network-unavailable` (*host network only*)
Adding these tolerations ensures backward compatibility. You can also add
arbitrary tolerations to DaemonSets.
## Device taints and tolerations
Instead of tainting entire nodes, administrators can also [taint individual devices](/docs/concepts/scheduling-eviction/dynamic-resource-allocation/#device-taints-and-tolerations)
when the cluster uses [dynamic resource allocation](/docs/concepts/scheduling-eviction/dynamic-resource-allocation/)
to manage special hardware. The advantage is that tainting can be targeted towards exactly the hardware that
is faulty or needs maintenance. Tolerations are also supported and can be specified when requesting
devices. Like taints they apply to all pods which share the same allocated device.
## What's next
* Read about [Node-pressure Eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/)
and how you can configure it
* Read about [Pod Priority](/docs/concepts/scheduling-eviction/pod-priority-preemption/)
* Read about [device taints and tolerations](/docs/concepts/scheduling-eviction/dynamic-resource-allocation/#device-taints-and-tolerations)
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified February 09, 2026 at 10:30 PM PST: [fix incorrect definition of Gt &amp; Lt operators in taints % tolerations (1a61fe63a4)](https://github.com/kubernetes/website/commit/1a61fe63a4cabfc4785c23ee0a0058fe6981d4bb)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Kubernetes Scheduler](docs-concepts-scheduling-eviction-kube-scheduler.md)
- [Концепции](ru-docs-concepts.md)
- [Certificates and Certificate Signing Requests](docs-reference-access-authn-authz-certificate-signing-requests.md)