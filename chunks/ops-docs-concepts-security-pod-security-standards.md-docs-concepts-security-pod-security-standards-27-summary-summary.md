---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#27-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 116
summary: * `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.user` * `spec.securityContext.seLinuxOptions.role` * `spec.containers[\*].securityContext.seLinuxOptions.role` *...
---

* `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.user`
* `spec.securityContext.seLinuxOptions.role`
* `spec.containers[\*].securityContext.seLinuxOptions.role`
* `spec.initContainers[\*].securityContext.seLinuxOptions.role`
* `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.role`
**Allowed Values**
* Undefined/""|
|`/proc` Mount Type|
The default `/proc` masks are set up to reduce attack surface, and should be required.
**Restricted Fields**