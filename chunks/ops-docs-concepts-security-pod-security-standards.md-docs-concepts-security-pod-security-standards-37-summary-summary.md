---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#37-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 124
summary: * `spec.containers[\*].securityContext.runAsNonRoot` * `spec.initContainers[\*].securityContext.runAsNonRoot` * `spec.ephemeralContainers[\*].securityContext.runAsNonRoot` **Allowed Values** *...
---

* `spec.containers[\*].securityContext.runAsNonRoot`
* `spec.initContainers[\*].securityContext.runAsNonRoot`
* `spec.ephemeralContainers[\*].securityContext.runAsNonRoot`
**Allowed Values**
* `true`The container fields may be undefined/`nil` if the pod-level
`spec.securityContext.runAsNonRoot` is set to `true`.|
|Running as Non-root user (v1.23+)|
Containers must not set runAsUser to 0
**Restricted Fields**
* `spec.securityContext.runAsUser`