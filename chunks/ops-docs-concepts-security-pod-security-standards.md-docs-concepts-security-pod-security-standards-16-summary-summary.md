---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#16-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 127
summary: * `spec.ephemeralContainers[\*].securityContext.privileged` **Allowed Values** * Undefined/nil * `false`| |Capabilities| Adding additional capabilities beyond those listed below must be disallowed....
---

* `spec.ephemeralContainers[\*].securityContext.privileged`
**Allowed Values**
* Undefined/nil
* `false`|
|Capabilities|
Adding additional capabilities beyond those listed below must be disallowed.
**Restricted Fields**
* `spec.containers[\*].securityContext.capabilities.add`
* `spec.initContainers[\*].securityContext.capabilities.add`
* `spec.ephemeralContainers[\*].securityContext.capabilities.add`
**Allowed Values**
* Undefined/nil
* `AUDIT\_WRITE`
* `CHOWN`
* `DAC\_OVERRIDE`
* `FOWNER`