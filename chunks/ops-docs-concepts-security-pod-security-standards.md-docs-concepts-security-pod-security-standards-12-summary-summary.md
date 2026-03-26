---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#12-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 126
summary: In this table, wildcards (`\*`) indicate all elements in a list. For example, `spec.containers[\*].securityContext` refers to the Security Context object for *all defined containers*. If any of the...
---

In this table, wildcards (`\*`) indicate all elements in a list. For example,
`spec.containers[\*].securityContext` refers to the Security Context object for *all defined
containers*. If any of the listed containers fails to meet the requirements, the entire pod will
fail validation.
Baseline policy specification|Control|Policy|
|HostProcess|
Windows Pods offer the ability to run [HostProcess containers](/docs/tasks/configure-pod-container/create-hostprocess-pod) which enables privileged access to the Windows host machine. Privileged access to the host is disallowed in the Baseline policy.