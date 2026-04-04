---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#7-summary
chunk_level: summary
chunk_type: prose
heading: Supported fields
token_count: 117
summary: ### Custom resources fields All custom resource types support the `metadata.name` and `metadata.namespace` fields. Additionally, the `spec.versions[\*].selectableFields` field of a...
---

### Custom resources fields
All custom resource types support the `metadata.name` and `metadata.namespace` fields.
Additionally, the `spec.versions[\*].selectableFields` field of a [CustomResourceDefinition](/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/)
declares which other fields in a custom resource may be used in field selectors. See [selectable fields for custom resources](/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#crd-selectable-fields)
for more information about how to use field selectors with CustomResourceDefinitions.