---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#4-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 490
summary: ### Example 3 This example uses the IP address range: 10.96.0.0/16 (CIDR notation) for the IP addresses of Services. Range Size: 216 - 2 = 65534 Band Offset: `min(max(16, 65536/16), 256)` =...
---

### Example 3
This example uses the IP address range: 10.96.0.0/16 (CIDR notation) for the IP addresses
of Services.
Range Size: 216 - 2 = 65534
Band Offset: `min(max(16, 65536/16), 256)` = `min(4096, 256)` = 256
Static band start: 10.96.0.1
Static band ends: 10.96.1.0
Range end: 10.96.255.254
pie showData
title 10.96.0.0/16
"Static" : 256
"Dynamic" : 65278
## What's next
* Read about [Service External Traffic Policy](/docs/tasks/access-application-cluster/create-external-load-balancer/#preserving-the-client-source-ip)
* Read about [Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/)
* Read about [Services](/docs/concepts/services-networking/service/)
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
Last modified October 24, 2024 at 5:03 PM PST: [Update cluster-ip-allocation.md (e57546d9a1)](https://github.com/kubernetes/website/commit/e57546d9a1a218b971ef1b19be8376b43db2d9e3)
## Related Pages

- [Концепции](ru-docs-concepts.md)
- [Tutorials](docs-tutorials.md)
- [Service](docs-reference-kubernetes-api-service-resources-service-v1.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)