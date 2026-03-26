---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#30-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: * `spec.securityContext.sysctls[\*].name` **Allowed Values** * Undefined/nil * `kernel.shm\_rmid\_forced` * `net.ipv4.ip\_local\_port\_range` * `net.ipv4.ip\_unprivileged\_port\_start` *...
---

* `spec.securityContext.sysctls[\*].name`
**Allowed Values**
* Undefined/nil
* `kernel.shm\_rmid\_forced`
* `net.ipv4.ip\_local\_port\_range`
* `net.ipv4.ip\_unprivileged\_port\_start`
* `net.ipv4.tcp\_syncookies`
* `net.ipv4.ping\_group\_range`
* `net.ipv4.ip\_local\_reserved\_ports` (since Kubernetes 1.27)
* `net.ipv4.tcp\_keepalive\_time` (since Kubernetes 1.29)