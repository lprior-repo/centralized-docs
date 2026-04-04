---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#47-summary
chunk_level: summary
chunk_type: table
heading: Types of volumes
token_count: 115
summary: ``` `!has(object.spec.volumes) || !object.spec.volumes.exists(v, has(v.gitRepo)) ` ``` You can use this deprecated storage plugin in your cluster if you explicitly enable the `GitRepoVolumeDriver`...
---

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