---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#2-standard
chunk_level: standard
chunk_type: table
heading: Supported fields
token_count: 422
summary: ## Supported fields Supported field selectors vary by Kubernetes resource type. All resource types support the `metadata.name` and `metadata.namespace` fields. Using unsupported field selectors...
---

## Supported fields
Supported field selectors vary by Kubernetes resource type. All resource types support the `metadata.name` and `metadata.namespace` fields. Using unsupported field selectors produces an error. For example:
```
`kubectl get ingress --field-selector foo.bar=baz
`
```
```
`Error from server (BadRequest): Unable to find "ingresses" that match label selector "", field selector "foo.bar=baz": "foo.bar" is not a known field selector: only "metadata.name", "metadata.namespace"
`
```
### List of supported fields
|Kind|Fields|
|Pod|`spec.nodeName`
`spec.restartPolicy`
`spec.schedulerName`
`spec.serviceAccountName`
`spec.hostNetwork`
`status.phase`
`status.podIP`
`status.podIPs`
`status.nominatedNodeName`|
|Event|`involvedObject.kind`
`involvedObject.namespace`
`involvedObject.name`
`involvedObject.uid`
`involvedObject.apiVersion`
`involvedObject.resourceVersion`
`involvedObject.fieldPath`
`reason`
`reportingComponent`
`source`
`type`|
|Secret|`type`|
|Namespace|`status.phase`|
|ReplicaSet|`status.replicas`|
|ReplicationController|`status.replicas`|
|Job|`status.successful`|
|Node|`spec.unschedulable`|
|CertificateSigningRequest|`spec.signerName`|
### Custom resources fields
All custom resource types support the `metadata.name` and `metadata.namespace` fields.
Additionally, the `spec.versions[\*].selectableFields` field of a [CustomResourceDefinition](/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/)
declares which other fields in a custom resource may be used in field selectors. See [selectable fields for custom resources](/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#crd-selectable-fields)
for more information about how to use field selectors with CustomResourceDefinitions.