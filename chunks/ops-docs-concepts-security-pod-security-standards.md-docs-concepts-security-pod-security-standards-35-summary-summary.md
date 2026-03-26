---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#35-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 117
summary: * `spec.volumes[\*].persistentVolumeClaim` * `spec.volumes[\*].projected` * `spec.volumes[\*].secret`| |Privilege Escalation (v1.8+)| Privilege escalation (such as via set-user-ID or set-group-ID...
---

* `spec.volumes[\*].persistentVolumeClaim`
* `spec.volumes[\*].projected`
* `spec.volumes[\*].secret`|
|Privilege Escalation (v1.8+)|
Privilege escalation (such as via set-user-ID or set-group-ID file mode) should not be allowed. *[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(spec.os.name != windows)`*
**Restricted Fields**
* `spec.containers[\*].securityContext.allowPrivilegeEscalation`