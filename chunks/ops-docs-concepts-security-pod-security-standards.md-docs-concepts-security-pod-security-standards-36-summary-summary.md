---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#36-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 124
summary: * `spec.containers[\*].securityContext.allowPrivilegeEscalation` * `spec.initContainers[\*].securityContext.allowPrivilegeEscalation` *...
---

* `spec.containers[\*].securityContext.allowPrivilegeEscalation`
* `spec.initContainers[\*].securityContext.allowPrivilegeEscalation`
* `spec.ephemeralContainers[\*].securityContext.allowPrivilegeEscalation`
**Allowed Values**
* `false`|
|Running as Non-root|
Containers must be required to run as non-root users.
**Restricted Fields**
* `spec.securityContext.runAsNonRoot`
* `spec.containers[\*].securityContext.runAsNonRoot`
* `spec.initContainers[\*].securityContext.runAsNonRoot`