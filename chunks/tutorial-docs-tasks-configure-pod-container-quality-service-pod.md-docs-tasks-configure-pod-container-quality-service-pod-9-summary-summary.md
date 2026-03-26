---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#9-summary
chunk_level: summary
chunk_type: prose
heading: Create a namespace
token_count: 111
summary: * [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all) * [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes) *...
---

* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
You also need to be able to create and delete namespaces.
## Create a namespace
Create a namespace so that the resources you create in this exercise are
isolated from the rest of your cluster.
```
`kubectl create namespace qos-example
`
```