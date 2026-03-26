---
doc_id: tutorial/docs-concepts-containers.md/docs-concepts-containers
chunk_id: tutorial/docs-concepts-containers.md/docs-concepts-containers#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 123
summary: Technology for packaging an application along with its runtime dependencies. This page will discuss containers and container images, as well as their use in operations and solution development. The...
---

Technology for packaging an application along with its runtime dependencies.
This page will discuss containers and container images, as well as their use in operations and solution development.
The word *container* is an overloaded term. Whenever you use the word, check whether your audience uses the same definition.
Each container that you run is repeatable; the standardization from having
dependencies included means that you get the same behavior wherever you
run it.
Containers decouple applications from the underlying host infrastructure.
This makes deployment easier in different cloud or OS environments.
Each [node](/docs/concepts/architecture/nodes/) in a Kubernetes