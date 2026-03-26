---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#25-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 126
summary: * Undefined/nil * `runtime/default` * `localhost/\*`| |SELinux| Setting the SELinux type is restricted, and setting a custom SELinux user or role option is forbidden. **Restricted Fields** *...
---

* Undefined/nil
* `runtime/default`
* `localhost/\*`|
|SELinux|
Setting the SELinux type is restricted, and setting a custom SELinux user or role option is forbidden.
**Restricted Fields**
* `spec.securityContext.seLinuxOptions.type`
* `spec.containers[\*].securityContext.seLinuxOptions.type`
* `spec.initContainers[\*].securityContext.seLinuxOptions.type`
* `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.type`
**Allowed Values**
* Undefined/""
* `container\_t`
* `container\_init\_t`