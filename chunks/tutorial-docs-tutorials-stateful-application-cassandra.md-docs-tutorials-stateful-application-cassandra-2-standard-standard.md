---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#2-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 398
summary: ## Objectives * Create and validate a Cassandra headless [Service](/docs/concepts/services-networking/service/). * Use a [StatefulSet](/docs/concepts/workloads/controllers/statefulset/) to create a...
---

## Objectives
* Create and validate a Cassandra headless [Service](/docs/concepts/services-networking/service/).
* Use a [StatefulSet](/docs/concepts/workloads/controllers/statefulset/) to create a Cassandra ring.
* Validate the StatefulSet.
* Modify the StatefulSet.
* Delete the StatefulSet and its [Pods](/docs/concepts/workloads/pods/).## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
To complete this tutorial, you should already have a basic familiarity with
[Pods](/docs/concepts/workloads/pods/),
[Services](/docs/concepts/services-networking/service/), and
[StatefulSets](/docs/concepts/workloads/controllers/statefulset/).
#### Caution:
[Minikube](https://minikube.sigs.k8s.io/docs/) defaults to 2048MB of memory and 2 CPU.
Running Minikube with the default resource configuration results in insufficient resource
errors during this tutorial. To avoid these errors, start Minikube with the following settings:
```
`minikube start --memory 5120 --cpus=4
`
```