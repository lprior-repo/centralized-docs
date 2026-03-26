---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#15-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 104
summary: * `spec.hostNetwork` * `spec.hostPID` * `spec.hostIPC` **Allowed Values** * Undefined/nil * `false`| |Privileged Containers| Privileged Pods disable most security mechanisms and must be disallowed....
---

* `spec.hostNetwork`
* `spec.hostPID`
* `spec.hostIPC`
**Allowed Values**
* Undefined/nil
* `false`|
|Privileged Containers|
Privileged Pods disable most security mechanisms and must be disallowed.
**Restricted Fields**
* `spec.containers[\*].securityContext.privileged`
* `spec.initContainers[\*].securityContext.privileged`
* `spec.ephemeralContainers[\*].securityContext.privileged`
**Allowed Values**
* Undefined/nil