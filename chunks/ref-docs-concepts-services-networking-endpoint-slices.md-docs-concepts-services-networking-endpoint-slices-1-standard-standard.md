---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 475
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