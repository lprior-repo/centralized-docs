---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#14-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 120
summary: ## Services and Labels A Service routes traffic across a set of Pods. Services are the abstraction that allows pods to die and replicate in Kubernetes without impacting your application. Discovery...
---

## Services and Labels
A Service routes traffic across a set of Pods. Services are the abstraction that allows
pods to die and replicate in Kubernetes without impacting your application. Discovery
and routing among dependent Pods (such as the frontend and backend components in an application)
are handled by Kubernetes Services.
Services match a set of Pods using
[labels and selectors](/docs/concepts/overview/working-with-objects/labels/), a grouping
primitive that allows logical operation on objects in Kubernetes. Labels are key/value
pairs attached to objects and can be used in any number of ways: