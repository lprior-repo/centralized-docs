---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#34-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 118
summary: * `spec.volumes[\*]` **Allowed Values** Every item in the `spec.volumes[\*]` list must set one of the following fields to a non-null value: * `spec.volumes[\*].configMap` * `spec.volumes[\*].csi` *...
---

* `spec.volumes[\*]`
**Allowed Values**
Every item in the `spec.volumes[\*]` list must set one of the following fields to a non-null value:
* `spec.volumes[\*].configMap`
* `spec.volumes[\*].csi`
* `spec.volumes[\*].downwardAPI`
* `spec.volumes[\*].emptyDir`
* `spec.volumes[\*].ephemeral`
* `spec.volumes[\*].persistentVolumeClaim`
* `spec.volumes[\*].projected`