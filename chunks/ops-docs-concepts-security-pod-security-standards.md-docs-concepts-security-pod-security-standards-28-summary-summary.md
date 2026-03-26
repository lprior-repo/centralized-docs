---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#28-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 123
summary: * `spec.containers[\*].securityContext.procMount` * `spec.initContainers[\*].securityContext.procMount` * `spec.ephemeralContainers[\*].securityContext.procMount` **Allowed Values** * Undefined/nil *...
---

* `spec.containers[\*].securityContext.procMount`
* `spec.initContainers[\*].securityContext.procMount`
* `spec.ephemeralContainers[\*].securityContext.procMount`
**Allowed Values**
* Undefined/nil
* `Default`|
|Seccomp|
Seccomp profile must not be explicitly set to `Unconfined`.
**Restricted Fields**
* `spec.securityContext.seccompProfile.type`
* `spec.containers[\*].securityContext.seccompProfile.type`
* `spec.initContainers[\*].securityContext.seccompProfile.type`