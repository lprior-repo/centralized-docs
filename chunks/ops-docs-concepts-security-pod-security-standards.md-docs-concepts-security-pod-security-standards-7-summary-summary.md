---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#7-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 127
summary: A detailed look at the different policy levels defined in the Pod Security Standards. The Pod Security Standards define three different *policies* to broadly cover the security spectrum. These...
---

A detailed look at the different policy levels defined in the Pod Security Standards.
The Pod Security Standards define three different *policies* to broadly cover the security
spectrum. These policies are *cumulative* and range from highly-permissive to highly-restrictive.
This guide outlines the requirements of each policy.
|Profile|Description|
|**Privileged**|Unrestricted policy, providing the widest possible level of permissions. This policy allows for known privilege escalations.|
|**Baseline**|Minimally restrictive policy which prevents known privilege escalations. Allows the default (minimally specified) Pod configuration.|
|