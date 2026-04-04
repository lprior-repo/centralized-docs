---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 122
summary: If this is specified and differs from the value in \"kind\", an equivalent match and conversion was performed. For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook...
---

If this is specified and differs from the value in "kind", an equivalent match and conversion was performed.
For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
`apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
an API request to apps/v1beta1 deployments would be converted and sent to the webhook
with `kind: {group:"apps", version:"v1", kind:"Deployment"}` (matching the rule the webhook registered for),
and