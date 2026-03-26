---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#5-summary
chunk_level: summary
chunk_type: prose
heading: Scaling an application
token_count: 106
summary: *You can create from the start a Deployment with multiple instances using the --replicas parameter for the kubectl create deployment command.* Previously we created a...
---

*You can create from the start a Deployment with multiple instances using the --replicas
parameter for the kubectl create deployment command.*
Previously we created a [Deployment](/docs/concepts/workloads/controllers/deployment/),
and then exposed it publicly via a [Service](/docs/concepts/services-networking/service/).
The Deployment created only one Pod for running our application. When traffic increases,
we will need to scale the application to keep up with user demand.
If you haven't worked through the earlier sections, start from