---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#8-summary
chunk_level: summary
chunk_type: prose
heading: Container hooks
token_count: 51
summary: * Exec - Executes a specific command, such as `pre-stop.sh`, inside the cgroups and namespaces of the Container. Resources consumed by the command are counted against the Container. * HTTP - Executes...
---

* Exec - Executes a specific command, such as `pre-stop.sh`, inside the cgroups and namespaces of the Container.
Resources consumed by the command are counted against the Container.
* HTTP - Executes an HTTP request against a specific endpoint on the Container.