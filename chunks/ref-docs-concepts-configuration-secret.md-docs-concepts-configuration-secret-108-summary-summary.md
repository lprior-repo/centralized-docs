---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#108-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 38
summary: 1. For each container in your Pod specification, add an environment variable for each Secret key that you want to use to the `env[].valueFrom.secretKeyRef` field.
---

1. For each container in your Pod specification, add an environment variable
for each Secret key that you want to use to the
`env[].valueFrom.secretKeyRef` field.