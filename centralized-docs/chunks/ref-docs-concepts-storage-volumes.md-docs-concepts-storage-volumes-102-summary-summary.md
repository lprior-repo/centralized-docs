---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#102-summary
chunk_level: summary
chunk_type: prose
heading: Using subPath
token_count: 128
summary: ## Using subPath Sometimes, it is useful to share one volume for multiple uses in a single Pod. The `volumeMounts[\*].subPath` property specifies a sub-path inside the referenced volume instead of...
---

## Using subPath
Sometimes, it is useful to share one volume for multiple uses in a single Pod.
The `volumeMounts[\*].subPath` property specifies a sub-path inside the referenced volume
instead of its root.
The following example shows how to configure a Pod with a LAMP stack (Linux, Apache, MySQL, PHP)
using a single, shared volume. This sample `subPath` configuration is not recommended
for production use.
The PHP application's code and assets map to the volume's `html` folder and
the MySQL database is stored in the volume's `mysql` folder. For example: