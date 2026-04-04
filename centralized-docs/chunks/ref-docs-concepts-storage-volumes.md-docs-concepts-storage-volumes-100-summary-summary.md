---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#100-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 125
summary: must be installed on the cluster. ### projected A projected volume maps several existing volume sources into the same directory. For more details, see [projected...
---

must be installed on the cluster.
### projected
A projected volume maps several existing volume sources into the same
directory. For more details, see [projected volumes](/docs/concepts/storage/projected-volumes/).
### secret
A `secret` volume is used to pass sensitive information, such as passwords, to
Pods. You can store secrets in the Kubernetes API and mount them as files for
use by Pods without coupling to Kubernetes directly. `secret` volumes are
backed by tmpfs (a RAM-backed filesystem), so they are never written to
non-volatile storage.
#### Note: