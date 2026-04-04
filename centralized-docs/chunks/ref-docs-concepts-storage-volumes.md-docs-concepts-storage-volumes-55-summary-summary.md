---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#55-summary
chunk_level: summary
chunk_type: table
heading: Types of volumes
token_count: 128
summary: * making a configuration file stored on the host system available read-only to a [static Pod](/docs/tasks/configure-pod-container/static-pod/); unlike normal Pods, static Pods cannot access...
---

* making a configuration file stored on the host system available read-only
to a [static Pod](/docs/tasks/configure-pod-container/static-pod/);
unlike normal Pods, static Pods cannot access ConfigMaps#### `hostPath` volume types
In addition to the required `path` property, you can optionally specify a
`type` for a `hostPath` volume.
The available values for `type` are:
|Value|Behavior|
|`‌""`|Empty string (default) is for backward compatibility, which means that no checks will be performed before mounting the `hostPath` volume.|
|