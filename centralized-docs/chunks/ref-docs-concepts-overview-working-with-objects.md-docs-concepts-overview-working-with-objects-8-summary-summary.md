---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#8-summary
chunk_level: summary
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 123
summary: [control plane](/docs/reference/glossary/?all=true#term-control-plane) continually and actively manages every object's actual state to match the desired state you supplied. For example: in...
---

[control plane](/docs/reference/glossary/?all=true#term-control-plane) continually
and actively manages every object's actual state to match the desired state you
supplied.
For example: in Kubernetes, a Deployment is an object that can represent an
application running on your cluster. When you create the Deployment, you
might set the Deployment `spec` to specify that you want three replicas of
the application to be running. The Kubernetes system reads the Deployment
spec and starts three instances of your desired application--updating
the status to match your spec. If any of those instances should fail