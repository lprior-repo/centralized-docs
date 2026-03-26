---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#14-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 116
summary: * `spec.securityContext.windowsOptions.hostProcess` * `spec.containers[\*].securityContext.windowsOptions.hostProcess` * `spec.initContainers[\*].securityContext.windowsOptions.hostProcess` *...
---

* `spec.securityContext.windowsOptions.hostProcess`
* `spec.containers[\*].securityContext.windowsOptions.hostProcess`
* `spec.initContainers[\*].securityContext.windowsOptions.hostProcess`
* `spec.ephemeralContainers[\*].securityContext.windowsOptions.hostProcess`
**Allowed Values**
* Undefined/nil
* `false`|
|Host Namespaces|
Sharing the host namespaces must be disallowed.
**Restricted Fields**
* `spec.hostNetwork`
* `spec.hostPID`
* `spec.hostIPC`
**Allowed Values**
* Undefined/nil