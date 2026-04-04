---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#11-standard
chunk_level: standard
chunk_type: table
heading: Types of volumes
token_count: 387
summary: #### Warning: The `gitRepo` volume plugin is deprecated and is disabled by default. To provision a Pod that has a Git repository mounted, you can mount an [`emptyDir`](#emptydir) volume into an [init...
---

#### Warning:
The `gitRepo` volume plugin is deprecated and is disabled by default.
To provision a Pod that has a Git repository mounted, you can mount an
[`emptyDir`](#emptydir) volume into an [init container](/docs/concepts/workloads/pods/init-containers/)
that clones the repo using Git, then mount the [EmptyDir](#emptydir) into the Pod's container.
You can restrict the use of `gitRepo` volumes in your cluster using
[policies](/docs/concepts/policy/), such as
[ValidatingAdmissionPolicy](/docs/reference/access-authn-authz/validating-admission-policy/).
You can use the following Common Expression Language (CEL) expression as
part of a policy to reject use of `gitRepo` volumes:
```
`!has(object.spec.volumes) || !object.spec.volumes.exists(v, has(v.gitRepo))
`
```
You can use this deprecated storage plugin in your cluster if you explicitly
enable the `GitRepoVolumeDriver`
[feature gate](/docs/reference/command-line-tools-reference/feature-gates/).
A `gitRepo` volume is an example of a volume plugin. This plugin
mounts an empty directory and clones a git repository into this directory
for your Pod to use.
Here is an example of a `gitRepo` volume:
```
`apiVersion: v1
kind: Pod
metadata:
name: server
spec:
containers:
- image: nginx
name: nginx
volumeMounts:
- mountPath: /mypath
name: git-volume
volumes:
- name: git-volume
gitRepo:
repository: "git@somewhere:me/my-git-repository.git"
revision: "22f1d8406d464b0c0874075539c1f2e96c253775"
`
```