---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 100
summary: ### Privileged **The *Privileged* policy is purposely-open, and entirely unrestricted.** This type of policy is typically aimed at system- and infrastructure-level workloads managed by privileged,...
---

### Privileged
**The *Privileged* policy is purposely-open, and entirely unrestricted.** This type of policy is
typically aimed at system- and infrastructure-level workloads managed by privileged, trusted users.
The Privileged policy is defined by an absence of restrictions. If you define a Pod where the Privileged
security policy applies, the Pod you define is able to bypass typical container isolation mechanisms.
For example, you can define a Pod that has access to the node's host network.