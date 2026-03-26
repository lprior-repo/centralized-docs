---
doc_id: tutorial/docs-concepts-containers.md/docs-concepts-containers
chunk_id: tutorial/docs-concepts-containers.md/docs-concepts-containers#5-summary
chunk_level: summary
chunk_type: prose
heading: Container images
token_count: 114
summary: A [container image](/docs/concepts/containers/images/) is a ready-to-run software package containing everything needed to run an application: the code and any runtime it requires, application and...
---

A [container image](/docs/concepts/containers/images/) is a ready-to-run
software package containing everything needed to run an application:
the code and any runtime it requires, application and system libraries,
and default values for any essential settings.
Containers are intended to be stateless and
[immutable](https://glossary.cncf.io/immutable-infrastructure/):
you should not change
the code of a container that is already running. If you have a containerized
application and want to make changes, the correct process is to build a new