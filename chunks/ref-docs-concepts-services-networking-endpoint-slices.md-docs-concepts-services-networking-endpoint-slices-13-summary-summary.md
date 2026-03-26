---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#13-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 126
summary: FEATURE STATE: `Kubernetes v1.26 [stable]` The `terminating` condition indicates that the endpoint is terminating. For endpoints backed by a Pod, this condition is set when the Pod is first deleted...
---

FEATURE STATE:
`Kubernetes v1.26 [stable]`
The `terminating` condition indicates that the endpoint is
terminating. For endpoints backed by a Pod, this condition is set when
the Pod is first deleted (that is, when it receives a deletion
timestamp, but most likely before the Pod's containers exit).
Service proxies will normally ignore endpoints that are `terminating`,
but they may route traffic to endpoints that are both `serving` and
`terminating` if all available endpoints are `terminating`. (This
helps to ensure that no Service traffic is lost during rolling updates