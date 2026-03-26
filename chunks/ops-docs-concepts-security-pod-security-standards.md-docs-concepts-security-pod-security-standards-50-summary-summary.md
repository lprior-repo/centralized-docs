---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#50-summary
chunk_level: summary
chunk_type: prose
heading: Policy Instantiation
token_count: 64
summary: ### Restricted Pod Security Standard changes Another important change, made in Kubernetes v1.25 is that the *Restricted* policy has been updated to use the `pod.spec.os.name` field. Based on the OS...
---

### Restricted Pod Security Standard changes
Another important change, made in Kubernetes v1.25 is that the *Restricted* policy
has been updated to use the `pod.spec.os.name` field. Based on the OS name, certain policies that are specific
to a particular OS can be relaxed for the other OS.