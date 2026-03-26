---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 793
summary: ### Duplicate endpoints Due to the nature of EndpointSlice changes, endpoints may be represented in more than one EndpointSlice at the same time. This naturally occurs as changes to different...
---

### Duplicate endpoints
Due to the nature of EndpointSlice changes, endpoints may be represented in more
than one EndpointSlice at the same time. This naturally occurs as changes to
different EndpointSlice objects can arrive at the Kubernetes client watch / cache
at different times.
#### Note:
Clients of the EndpointSlice API must iterate through all the existing EndpointSlices
associated to a Service and build a complete list of unique network endpoints. It is
important to mention that endpoints may be duplicated in different EndpointSlices.
You can find a reference implementation for how to perform this endpoint aggregation
and deduplication as part of the `EndpointSliceCache` code within `kube-proxy`.
### EndpointSlice mirroring
FEATURE STATE:
`Kubernetes v1.33 [deprecated]`
The EndpointSlice API is a replacement for the older Endpoints API. To
preserve compatibility with older controllers and user workloads that
expect [kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/)
to route traffic based on Endpoints resources, the cluster's control
plane mirrors most user-created Endpoints resources to corresponding
EndpointSlices.
(However, this feature, like the rest of the Endpoints API, is
deprecated. Users who manually specify endpoints for selectorless
Services should do so by creating EndpointSlice resources directly,
rather than by creating Endpoints resources and allowing them to be
mirrored.)
The control plane mirrors Endpoints resources unless:
* the Endpoints resource has a `endpointslice.kubernetes.io/skip-mirror` label
set to `true`.
* the Endpoints resource has a `control-plane.alpha.kubernetes.io/leader`
annotation.
* the corresponding Service resource does not exist.
* the corresponding Service resource has a non-nil selector.
Individual Endpoints resources may translate into multiple EndpointSlices. This
will occur if an Endpoints resource has multiple subsets or includes endpoints
with multiple IP families (IPv4 and IPv6). A maximum of 1000 addresses per
subset will be mirrored to EndpointSlices.
## What's next
* Follow the [Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/) tutorial
* Read the [API reference](/docs/reference/kubernetes-api/service-resources/endpoint-slice-v1/) for the EndpointSlice API
* Read the [API reference](/docs/reference/kubernetes-api/service-resources/endpoints-v1/) for the Endpoints API
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
Last modified June 22, 2025 at 4:42 PM PST: [Improve glossary entry for EndpointSlice (5fadc4a1b3)](https://github.com/kubernetes/website/commit/5fadc4a1b30559723ab52e18e678b46a092de848)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [expose intro](docs-tutorials-kubernetes-basics-expose-expose-intro.md)
- [Use a Service to Access an Application in a Cluster](docs-tasks-access-application-cluster-service-access-application-cluster.md)
- [Secrets](docs-concepts-configuration-secret.md)
- [Owners and Dependents](docs-concepts-overview-working-with-objects-owners-dependents.md)