---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#42-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 125
summary: *[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(.spec.os.name != \"windows\")`* **Restricted Fields** * `spec.containers[\*].securityContext.capabilities.drop` *...
---

*[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(.spec.os.name != "windows")`*
**Restricted Fields**
* `spec.containers[\*].securityContext.capabilities.drop`
* `spec.initContainers[\*].securityContext.capabilities.drop`
* `spec.ephemeralContainers[\*].securityContext.capabilities.drop`
**Allowed Values**
* Any list of capabilities that includes `ALL`
**Restricted Fields**
* `spec.containers[\*].securityContext.capabilities.add`
* `spec.initContainers[\*].securityContext.capabilities.add`