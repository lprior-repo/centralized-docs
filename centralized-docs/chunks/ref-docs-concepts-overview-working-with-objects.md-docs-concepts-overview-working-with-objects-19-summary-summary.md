---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#19-summary
chunk_level: summary
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 127
summary: Within the `.spec` of a StatefulSet is a [template](/docs/concepts/workloads/pods/#pod-templates) for Pod objects. That template describes Pods that the StatefulSet controller will create in order to...
---

Within the `.spec` of a StatefulSet is a [template](/docs/concepts/workloads/pods/#pod-templates)
for Pod objects. That template describes Pods that the StatefulSet controller will create in order to
satisfy the StatefulSet specification.
Different kinds of objects can also have different `.status`; again, the API reference pages
detail the structure of that `.status` field, and its content for each different type of object.
See [Kubernetes Configuration Best Practices](/blog/2025/11/25/configuration-good-practices/) for additional
information on writing YAML configuration files.