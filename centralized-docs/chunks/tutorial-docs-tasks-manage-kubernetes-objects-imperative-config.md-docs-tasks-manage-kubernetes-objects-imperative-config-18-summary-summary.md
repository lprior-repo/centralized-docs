---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#18-summary
chunk_level: summary
chunk_type: prose
heading: Migrating from imperative commands to imperative object configuration
token_count: 78
summary: #### Warning: Updating selectors on controllers is strongly discouraged. The recommended approach is to define a single, immutable PodTemplate label used only by the controller selector with no other...
---

#### Warning:
Updating selectors on controllers is strongly discouraged.
The recommended approach is to define a single, immutable PodTemplate label
used only by the controller selector with no other semantic meaning.
Example label:
```
`selector:
matchLabels:
controller-selector: "apps/v1/deployment/nginx"
template:
metadata:
labels:
controller-selector: "apps/v1/deployment/nginx"
`
```