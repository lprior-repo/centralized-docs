---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#99-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 58
summary: By default, Secrets are required. None of a Pod's containers will start until all non-optional Secrets are available. If a Pod references a specific key in a non-optional Secret and that Secret does...
---

By default, Secrets are required. None of a Pod's containers will start until
all non-optional Secrets are available.
If a Pod references a specific key in a non-optional Secret and that Secret
does exist, but is missing the named key, the Pod fails during startup.