---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#35-summary
chunk_level: summary
chunk_type: prose
heading: `PolicyRule`
token_count: 79
summary: * a value of 'true' will drop the managed fields from the API audit log * a value of 'false' indicates that the managed fileds should be included in the API audit log Note that the value, if...
---

* a value of 'true' will drop the managed fields from the API audit log
* a value of 'false' indicates that the managed fileds should be included
in the API audit log
Note that the value, if specified, in this rule will override the global default
If a value is not specified then the global default specified in
Policy.OmitManagedFields will stand.|