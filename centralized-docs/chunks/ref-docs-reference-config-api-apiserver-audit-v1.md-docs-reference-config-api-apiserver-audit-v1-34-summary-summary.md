---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#34-summary
chunk_level: summary
chunk_type: prose
heading: `PolicyRule`
token_count: 118
summary: * `/healthz\*` - Log all health checks| |`omitStages` [`[]Stage`](#audit-k8s-io-v1-Stage)| OmitStages is a list of stages for which no events are created. Note that this can also be specified policy...
---

* `/healthz\*` - Log all health checks|
|`omitStages`
[`[]Stage`](#audit-k8s-io-v1-Stage)|
OmitStages is a list of stages for which no events are created. Note that this can also
be specified policy wide in which case the union of both are omitted.
An empty list means no restrictions will apply.
|
|`omitManagedFields`
`bool`|
OmitManagedFields indicates whether to omit the managed fields of the request
and response bodies from being written to the API audit log.