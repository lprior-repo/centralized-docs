---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 361
summary: ### Address types EndpointSlices support two address types: * IPv4 * IPv6 Each `EndpointSlice` object represents a specific IP address type. If you have a Service that is available via IPv4 and IPv6,...
---

### Address types
EndpointSlices support two address types:
* IPv4
* IPv6
Each `EndpointSlice` object represents a specific IP address type. If you have
a Service that is available via IPv4 and IPv6, there will be at least two
`EndpointSlice` objects (one for IPv4, and one for IPv6).
### Conditions
The EndpointSlice API stores conditions about endpoints that may be useful for consumers.
The three conditions are `serving`, `terminating`, and `ready`.
#### Serving
FEATURE STATE:
`Kubernetes v1.26 [stable]`
The `serving` condition indicates that the endpoint is currently serving responses, and
so it should be used as a target for Service traffic. For endpoints backed by a Pod, this
maps to the Pod's `Ready` condition.
#### Terminating
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
of the underlying Pods.)
#### Ready
The `ready` condition is essentially a shortcut for checking
"`serving` and not `terminating`" (though it will also always be
`true` for Services with `spec.publishNotReadyAddresses` set to
`true`).