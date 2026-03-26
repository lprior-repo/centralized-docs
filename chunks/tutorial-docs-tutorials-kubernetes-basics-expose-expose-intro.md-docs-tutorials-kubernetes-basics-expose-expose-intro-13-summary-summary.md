---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#13-summary
chunk_level: summary
chunk_type: prose
heading: Overview of Kubernetes Services
token_count: 97
summary: tutorial. Also see [Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/). Additionally, note that there are some use cases with Services that involve not...
---

 tutorial. Also see
[Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/).
Additionally, note that there are some use cases with Services that involve not defining
a `selector` in the spec. A Service created without `selector` will also not create
the corresponding Endpoints object. This allows users to manually map a Service to
specific endpoints. Another possibility why there may be no selector is you are strictly
using `type: ExternalName`.