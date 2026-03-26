---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#24-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 98
summary: * `spec.containers[\*].securityContext.appArmorProfile.type` * `spec.initContainers[\*].securityContext.appArmorProfile.type` * `spec.ephemeralContainers[\*].securityContext.appArmorProfile.type`...
---

* `spec.containers[\*].securityContext.appArmorProfile.type`
* `spec.initContainers[\*].securityContext.appArmorProfile.type`
* `spec.ephemeralContainers[\*].securityContext.appArmorProfile.type`
**Allowed Values**
* Undefined/nil
* `RuntimeDefault`
* `Localhost`
* `metadata.annotations["container.apparmor.security.beta.kubernetes.io/\*"]`
**Allowed Values**
* Undefined/nil
* `runtime/default`