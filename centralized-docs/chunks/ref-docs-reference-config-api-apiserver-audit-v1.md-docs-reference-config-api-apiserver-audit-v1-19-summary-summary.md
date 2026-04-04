---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#19-summary
chunk_level: summary
chunk_type: prose
heading: `Policy`
token_count: 60
summary: This is used as a global default - a value of 'true' will omit the managed fileds, otherwise the managed fields will be included in the API audit log. Note that this can also be specified per rule in...
---

This is used as a global default - a value of 'true' will omit the managed fileds,
otherwise the managed fields will be included in the API audit log.
Note that this can also be specified per rule in which case the value specified
in a rule will override the global default.
|