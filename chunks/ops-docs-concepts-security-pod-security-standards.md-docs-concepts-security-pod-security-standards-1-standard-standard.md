---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#1-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 251
summary: # Pod Security Standards A detailed look at the different policy levels defined in the Pod Security Standards. The Pod Security Standards define three different *policies* to broadly cover the...
---

# Pod Security Standards
A detailed look at the different policy levels defined in the Pod Security Standards.
The Pod Security Standards define three different *policies* to broadly cover the security
spectrum. These policies are *cumulative* and range from highly-permissive to highly-restrictive.
This guide outlines the requirements of each policy.
|Profile|Description|
|**Privileged**|Unrestricted policy, providing the widest possible level of permissions. This policy allows for known privilege escalations.|
|**Baseline**|Minimally restrictive policy which prevents known privilege escalations. Allows the default (minimally specified) Pod configuration.|
|**Restricted**|Heavily restricted policy, following current Pod hardening best practices.|
### Privileged
**The *Privileged* policy is purposely-open, and entirely unrestricted.** This type of policy is
typically aimed at system- and infrastructure-level workloads managed by privileged, trusted users.
The Privileged policy is defined by an absence of restrictions. If you define a Pod where the Privileged
security policy applies, the Pod you define is able to bypass typical container isolation mechanisms.
For example, you can define a Pod that has access to the node's host network.