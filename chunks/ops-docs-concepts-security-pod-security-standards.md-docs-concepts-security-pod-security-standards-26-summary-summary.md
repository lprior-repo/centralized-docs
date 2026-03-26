---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#26-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 115
summary: * Undefined/\"\" * `container\_t` * `container\_init\_t` * `container\_kvm\_t` * `container\_engine\_t` (since Kubernetes 1.31) **Restricted Fields** * `spec.securityContext.seLinuxOptions.user` *...
---

* Undefined/""
* `container\_t`
* `container\_init\_t`
* `container\_kvm\_t`
* `container\_engine\_t` (since Kubernetes 1.31)
**Restricted Fields**
* `spec.securityContext.seLinuxOptions.user`
* `spec.containers[\*].securityContext.seLinuxOptions.user`
* `spec.initContainers[\*].securityContext.seLinuxOptions.user`
* `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.user`
* `spec.securityContext.seLinuxOptions.role`