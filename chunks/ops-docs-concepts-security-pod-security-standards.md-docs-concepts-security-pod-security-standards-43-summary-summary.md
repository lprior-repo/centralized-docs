---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#43-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 63
summary: * `spec.containers[\*].securityContext.capabilities.add` * `spec.initContainers[\*].securityContext.capabilities.add` * `spec.ephemeralContainers[\*].securityContext.capabilities.add` **Allowed...
---

* `spec.containers[\*].securityContext.capabilities.add`
* `spec.initContainers[\*].securityContext.capabilities.add`
* `spec.ephemeralContainers[\*].securityContext.capabilities.add`
**Allowed Values**
* Undefined/nil
* `NET\_BIND\_SERVICE`|