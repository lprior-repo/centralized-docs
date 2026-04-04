---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#16-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 426
summary: ## Device taints and tolerations Instead of tainting entire nodes, administrators can also [taint individual...
---

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