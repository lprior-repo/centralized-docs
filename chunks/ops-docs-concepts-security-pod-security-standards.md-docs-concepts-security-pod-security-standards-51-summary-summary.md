---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#51-summary
chunk_level: summary
chunk_type: prose
heading: Policy Instantiation
token_count: 110
summary: #### OS-specific policy controls Restrictions on the following controls are only required if `.spec.os.name` is not `windows`: * Privilege Escalation * Seccomp * Linux Capabilities## User namespaces...
---

#### OS-specific policy controls
Restrictions on the following controls are only required if `.spec.os.name` is not `windows`:
* Privilege Escalation
* Seccomp
* Linux Capabilities## User namespaces
User Namespaces are a Linux-only feature to run workloads with increased
isolation. How they work together with Pod Security Standards is described in
the [documentation](/docs/concepts/workloads/pods/user-namespaces/#integration-with-pod-security-admission-checks) for Pods that use user namespaces.