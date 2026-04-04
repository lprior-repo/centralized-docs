---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#56-summary
chunk_level: summary
chunk_type: prose
heading: Device taints and tolerations
token_count: 128
summary: ## Device taints and tolerations Instead of tainting entire nodes, administrators can also [taint individual...
---

## Device taints and tolerations
Instead of tainting entire nodes, administrators can also [taint individual devices](/docs/concepts/scheduling-eviction/dynamic-resource-allocation/#device-taints-and-tolerations)
when the cluster uses [dynamic resource allocation](/docs/concepts/scheduling-eviction/dynamic-resource-allocation/)
to manage special hardware. The advantage is that tainting can be targeted towards exactly the hardware that
is faulty or needs maintenance. Tolerations are also supported and can be specified when requesting
devices. Like taints they apply to all pods which share the same allocated device.