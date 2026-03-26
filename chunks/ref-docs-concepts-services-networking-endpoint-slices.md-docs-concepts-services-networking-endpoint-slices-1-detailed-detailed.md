---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 836
summary: # EndpointSlices The EndpointSlice API is the mechanism that Kubernetes uses to let your Service scale to handle large numbers of backends, and allows the cluster to update its list of healthy...
---

# EndpointSlices
The EndpointSlice API is the mechanism that Kubernetes uses to let your Service scale to handle large numbers of backends, and allows the cluster to update its list of healthy backends efficiently.
FEATURE STATE:
`Kubernetes v1.21 [stable]`
EndpointSlices track the IP addresses of backend endpoints.
EndpointSlices are normally associated with a
[Service](/docs/concepts/services-networking/service/) and the backend endpoints typically represent
[Pods](/docs/concepts/workloads/pods/).## EndpointSlice API
In Kubernetes, an EndpointSlice contains references to a set of network
endpoints. The control plane automatically creates EndpointSlices
for any Kubernetes Service that has a [selector](/docs/concepts/overview/working-with-objects/labels/) specified. These EndpointSlices include
references to all the Pods that match the Service selector. EndpointSlices group
network endpoints together by unique combinations of IP family, protocol,
port number, and Service name.
The name of a EndpointSlice object must be a valid
[DNS subdomain name](/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names).
As an example, here's a sample EndpointSlice object, that's owned by the `example`
Kubernetes Service.
```
`apiVersion: discovery.k8s.io/v1
kind: EndpointSlice
metadata:
name: example-abc
labels:
kubernetes.io/service-name: example
addressType: IPv4
ports:
- name: http
protocol: TCP
port: 80
endpoints:
- addresses:
- "10.1.2.3"
conditions:
ready: true
hostname: pod-1
nodeName: node-1
zone: us-west2-a
`
```
By default, the control plane creates and manages EndpointSlices to have no
more than 100 endpoints each. You can configure this with the
`--max-endpoints-per-slice`
[kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/)
flag, up to a maximum of 1000.
EndpointSlices act as the source of truth for
[kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/) when it comes to
how to route internal traffic.
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