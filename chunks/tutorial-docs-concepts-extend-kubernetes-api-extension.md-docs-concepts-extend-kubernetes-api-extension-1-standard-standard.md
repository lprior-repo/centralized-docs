---
doc_id: tutorial/docs-concepts-extend-kubernetes-api-extension.md/docs-concepts-extend-kubernetes-api-extension
chunk_id: tutorial/docs-concepts-extend-kubernetes-api-extension.md/docs-concepts-extend-kubernetes-api-extension#1-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 400
summary: # Extending the Kubernetes API Custom resources are extensions of the Kubernetes API. Kubernetes provides two ways to add custom resources to your cluster: * The...
---

# Extending the Kubernetes API
Custom resources are extensions of the Kubernetes API. Kubernetes provides two ways to add custom resources to your cluster:
* The [CustomResourceDefinition](/docs/concepts/extend-kubernetes/api-extension/custom-resources/)
(CRD) mechanism allows you to declaratively define a new custom API with an API group, kind, and
schema that you specify.
The Kubernetes control plane serves and handles the storage of your custom resource. CRDs allow you to
create new types of resources for your cluster without writing and running a custom API server.
* The [aggregation layer](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/)
sits behind the primary API server, which acts as a proxy.
This arrangement is called API Aggregation (AA), which allows you to provide
specialized implementations for your custom resources by writing and
deploying your own API server.
The main API server delegates requests to your API server for the custom APIs that you specify,
making them available to all of its clients.
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
Last modified January 26, 2024 at 11:11 PM PST: [update space (dad0076e02)](https://github.com/kubernetes/website/commit/dad0076e026cf684bf6d4b37fc3b5e6b3335c8cb)