---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Server side field validation
token_count: 663
summary: ### Required fields In the manifest (YAML or JSON file) for the Kubernetes object you want to create, you'll need to set values for the following fields: * `apiVersion` - Which version of the...
---

### Required fields
In the manifest (YAML or JSON file) for the Kubernetes object you want to create, you'll need to set values for
the following fields:
* `apiVersion` - Which version of the Kubernetes API you're using to create this object
* `kind` - What kind of object you want to create
* `metadata` - Data that helps uniquely identify the object, including a `name` string, `UID`, and optional `namespace`
* `spec` - What state you desire for the object
The precise format of the object `spec` is different for every Kubernetes object, and contains
nested fields specific to that object. The [Kubernetes API Reference](/docs/reference/kubernetes-api/)
can help you find the spec format for all of the objects you can create using Kubernetes.
For example, see the [`spec` field](/docs/reference/kubernetes-api/workload-resources/pod-v1/#PodSpec)
for the Pod API reference.
For each Pod, the `.spec` field specifies the pod and its desired state (such as the container image name for
each container within that pod).
Another example of an object specification is the
[`spec` field](/docs/reference/kubernetes-api/workload-resources/stateful-set-v1/#StatefulSetSpec)
for the StatefulSet API. For StatefulSet, the `.spec` field specifies the StatefulSet and
its desired state.
Within the `.spec` of a StatefulSet is a [template](/docs/concepts/workloads/pods/#pod-templates)
for Pod objects. That template describes Pods that the StatefulSet controller will create in order to
satisfy the StatefulSet specification.
Different kinds of objects can also have different `.status`; again, the API reference pages
detail the structure of that `.status` field, and its content for each different type of object.
See [Kubernetes Configuration Best Practices](/blog/2025/11/25/configuration-good-practices/) for additional
information on writing YAML configuration files.
## Server side field validation
Starting with Kubernetes v1.25, the API server offers server side
[field validation](/docs/reference/using-api/api-concepts/#field-validation)
that detects unrecognized or duplicate fields in an object. It provides all the functionality
of `kubectl --validate` on the server side.
The `kubectl` tool uses the `--validate` flag to set the level of field validation. It accepts the
values `ignore`, `warn`, and `strict` while also accepting the values `true` (equivalent to `strict`)
and `false` (equivalent to `ignore`). The default validation setting for `kubectl` is `--validate=true`.
`Strict`Strict field validation, errors on validation failure`Warn`Field validation is performed, but errors are exposed as warnings rather than failing the request`Ignore`No server side field validation is performed
When `kubectl` cannot connect to an API server that supports field validation it will fall back
to using client-side validation. Kubernetes 1.27 and later versions always offer field validation;
older Kubernetes releases might not. If your cluster is older than v1.27, check the documentation
for your version of Kubernetes.