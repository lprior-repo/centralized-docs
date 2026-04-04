---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#3-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 105
summary: * [EventList](#audit-k8s-io-v1-EventList) Event captures all the information that can be included in an API audit log. |Field|Description| |`apiVersion` string|`audit.k8s.io/v1`| |`kind`...
---

* [EventList](#audit-k8s-io-v1-EventList)
Event captures all the information that can be included in an API audit log.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`Event`|
|`level`**[Required]**
[`Level`](#audit-k8s-io-v1-Level)|
AuditLevel at which event was generated
|
|`auditID`**[Required]**