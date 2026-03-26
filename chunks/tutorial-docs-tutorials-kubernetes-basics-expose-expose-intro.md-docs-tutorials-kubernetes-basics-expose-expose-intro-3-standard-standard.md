---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#3-standard
chunk_level: standard
chunk_type: prose
heading: Services and Labels
token_count: 197
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
* Designate objects for development, test, and production
* Embed version tags
* Classify an object using tags![](/docs/tutorials/kubernetes-basics/public/images/module_04_labels.svg)
Labels can be attached to objects at creation time or later on. They can be modified
at any time. Let's expose our application now using a Service and apply some labels.