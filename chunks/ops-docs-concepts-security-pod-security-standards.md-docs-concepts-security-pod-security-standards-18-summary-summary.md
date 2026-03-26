---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#18-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 101
summary: * `spec.volumes[\*].hostPath` **Allowed Values** * Undefined/nil| |Host Ports| HostPorts should be disallowed entirely (recommended) or restricted to a known list **Restricted Fields** *...
---

* `spec.volumes[\*].hostPath`
**Allowed Values**
* Undefined/nil|
|Host Ports|
HostPorts should be disallowed entirely (recommended) or restricted to a known list
**Restricted Fields**
* `spec.containers[\*].ports[\*].hostPort`
* `spec.initContainers[\*].ports[\*].hostPort`
* `spec.ephemeralContainers[\*].ports[\*].hostPort`
**Allowed Values**
* Undefined/nil