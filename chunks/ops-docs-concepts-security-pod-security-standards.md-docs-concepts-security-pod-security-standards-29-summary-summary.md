---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#29-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 125
summary: * `spec.initContainers[\*].securityContext.seccompProfile.type` * `spec.ephemeralContainers[\*].securityContext.seccompProfile.type` **Allowed Values** * Undefined/nil * `RuntimeDefault` *...
---

* `spec.initContainers[\*].securityContext.seccompProfile.type`
* `spec.ephemeralContainers[\*].securityContext.seccompProfile.type`
**Allowed Values**
* Undefined/nil
* `RuntimeDefault`
* `Localhost`|
|Sysctls|
Sysctls can disable security mechanisms or affect all containers on a host, and should be disallowed except for an allowed "safe" subset. A sysctl is considered safe if it is namespaced in the container or the Pod, and it is isolated from other Pods or processes on the same Node.
**Restricted Fields**