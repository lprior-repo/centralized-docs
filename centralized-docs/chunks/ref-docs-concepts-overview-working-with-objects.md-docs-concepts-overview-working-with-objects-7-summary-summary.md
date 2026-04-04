---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#7-summary
chunk_level: summary
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 128
summary: Almost every Kubernetes object includes two nested object fields that govern the object's configuration: the object *`spec`* and the object *`status`*. For objects that have a `spec`, you have to set...
---

Almost every Kubernetes object includes two nested object fields that govern
the object's configuration: the object *`spec`* and the object *`status`*.
For objects that have a `spec`, you have to set this when you create the object,
providing a description of the characteristics you want the resource to have:
its *desired state*.
The `status` describes the *current state* of the object, supplied and updated
by the Kubernetes system and its components. The Kubernetes
[control plane](/docs/reference/glossary/?all=true#term-control-plane) continually
and actively manages every object'