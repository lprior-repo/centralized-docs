---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#18-summary
chunk_level: summary
chunk_type: prose
heading: `Policy`
token_count: 124
summary: PolicyRules are strictly ordered. | |`omitStages` [`[]Stage`](#audit-k8s-io-v1-Stage)| OmitStages is a list of stages for which no events are created. Note that this can also be specified per rule in...
---

PolicyRules are strictly ordered.
|
|`omitStages`
[`[]Stage`](#audit-k8s-io-v1-Stage)|
OmitStages is a list of stages for which no events are created. Note that this can also
be specified per rule in which case the union of both are omitted.
|
|`omitManagedFields`
`bool`|
OmitManagedFields indicates whether to omit the managed fields of the request
and response bodies from being written to the API audit log.
This is used as a global default - a value of 'true' will omit the managed fileds,