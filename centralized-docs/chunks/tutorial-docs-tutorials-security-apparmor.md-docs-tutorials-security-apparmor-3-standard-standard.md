---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#3-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 248
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
* `RuntimeDefault` to use the runtime's default profile
* `Localhost` to use a profile loaded on the host (see below)
* `Unconfined` to run without AppArmor
See [Specifying AppArmor Confinement](#specifying-apparmor-confinement) for full details on the AppArmor profile API.
To verify that the profile was applied, you can check that the container's root process is
running with the correct profile by examining its proc attr:
```
`kubectl exec &lt;pod\_name&gt; -- cat /proc/1/attr/current
`
```
The output should look something like this:
```
`cri-containerd.apparmor.d (enforce)
`
```