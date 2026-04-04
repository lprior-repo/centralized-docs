---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#12-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 125
summary: `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]` and `matchPolicy: Equivalent`, an API request to apps/v1beta1 deployments would be converted and sent to the webhook with...
---

`apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
an API request to apps/v1beta1 deployments would be converted and sent to the webhook
with `resource: {group:"apps", version:"v1", resource:"deployments"}` (matching the resource the webhook registered for),
and `requestResource: {group:"apps", version:"v1beta1", resource:"deployments"}` (indicating the resource of the original API request).
See documentation for the "matchPolicy" field in the webhook configuration type.
|
|