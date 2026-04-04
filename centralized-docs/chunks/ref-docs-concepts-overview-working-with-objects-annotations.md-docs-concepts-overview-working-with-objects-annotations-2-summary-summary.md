---
doc_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations
chunk_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations#2-summary
chunk_level: summary
chunk_type: prose
heading: Attaching metadata to objects
token_count: 111
summary: ## Attaching metadata to objects You can use either labels or annotations to attach metadata to Kubernetes objects. Labels can be used to select objects and to find collections of objects that...
---

## Attaching metadata to objects
You can use either labels or annotations to attach metadata to Kubernetes
objects. Labels can be used to select objects and to find
collections of objects that satisfy certain conditions. In contrast, annotations
are not used to identify and select objects. The metadata
in an annotation can be small or large, structured or unstructured, and can
include characters not permitted by labels. It is possible to use labels as
well as annotations in the metadata of the same object.
Annotations, like labels, are key/value maps: