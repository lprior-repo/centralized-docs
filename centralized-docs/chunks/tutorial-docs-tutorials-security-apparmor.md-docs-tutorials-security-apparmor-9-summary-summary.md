---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#9-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 98
summary: #### Note: Prior to Kubernetes v1.30, AppArmor was specified through annotations. Use the documentation version selector to view the documentation with this deprecated API. AppArmor profiles can be...
---

#### Note:
Prior to Kubernetes v1.30, AppArmor was specified through annotations. Use the documentation version
selector to view the documentation with this deprecated API.
AppArmor profiles can be specified at the pod level or container level. The container AppArmor
profile takes precedence over the pod profile.
```
`securityContext:
appArmorProfile:
type: &lt;profile\_type&gt;
`
```
Where `&lt;profile\_type&gt;` is one of: