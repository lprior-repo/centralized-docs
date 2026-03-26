---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#33-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 106
summary: #### Note: In this table, wildcards (`\*`) indicate all elements in a list. For example, `spec.containers[\*].securityContext` refers to the Security Context object for *all defined containers*. If...
---

#### Note:
In this table, wildcards (`\*`) indicate all elements in a list. For example,
`spec.containers[\*].securityContext` refers to the Security Context object for *all defined
containers*. If any of the listed containers fails to meet the requirements, the entire pod will
fail validation.
Restricted policy specification|**Control**|**Policy**|
|*Everything from the Baseline policy*|
|Volume Types|
The Restricted policy only permits the following volume types.
**Restricted Fields**