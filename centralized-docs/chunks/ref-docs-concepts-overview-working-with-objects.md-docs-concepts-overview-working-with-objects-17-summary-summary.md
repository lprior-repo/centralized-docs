---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#17-summary
chunk_level: summary
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 126
summary: * `spec` - What state you desire for the object The precise format of the object `spec` is different for every Kubernetes object, and contains nested fields specific to that object. The [Kubernetes...
---

* `spec` - What state you desire for the object
The precise format of the object `spec` is different for every Kubernetes object, and contains
nested fields specific to that object. The [Kubernetes API Reference](/docs/reference/kubernetes-api/)
can help you find the spec format for all of the objects you can create using Kubernetes.
For example, see the [`spec` field](/docs/reference/kubernetes-api/workload-resources/pod-v1/#PodSpec)
for the Pod API reference.
For each Pod, the `.spec` field specifies the pod and its desired state (such as the container image name for