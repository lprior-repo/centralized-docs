---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Creating a headless Service for Cassandra
token_count: 979
summary: # Example: Deploying Cassandra with a StatefulSet This tutorial shows you how to run [Apache Cassandra](https://cassandra.apache.org/) on Kubernetes. Cassandra, a database, needs persistent storage...
---

# Example: Deploying Cassandra with a StatefulSet
This tutorial shows you how to run [Apache Cassandra](https://cassandra.apache.org/) on Kubernetes.
Cassandra, a database, needs persistent storage to provide data durability (application *state*).
In this example, a custom Cassandra seed provider lets the database discover new Cassandra instances as they join the Cassandra cluster.
*StatefulSets* make it easier to deploy stateful applications into your Kubernetes cluster.
For more information on the features used in this tutorial, see
[StatefulSet](/docs/concepts/workloads/controllers/statefulset/).
#### Note:
Cassandra and Kubernetes both use the term *node* to mean a member of a cluster. In this
tutorial, the Pods that belong to the StatefulSet are Cassandra nodes and are members
of the Cassandra cluster (called a *ring*). When those Pods run in your Kubernetes cluster,
the Kubernetes control plane schedules those Pods onto Kubernetes
[Nodes](/docs/concepts/architecture/nodes/).
When a Cassandra node starts, it uses a *seed list* to bootstrap discovery of other
nodes in the ring.
This tutorial deploys a custom Cassandra seed provider that lets the database discover
new Cassandra Pods as they appear inside your Kubernetes cluster.
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
## Creating a headless Service for Cassandra
In Kubernetes, a [Service](/docs/concepts/services-networking/service/) describes a set of
[Pods](/docs/concepts/workloads/pods/) that perform the same task.
The following Service is used for DNS lookups between Cassandra Pods and clients within your cluster:
[`application/cassandra/cassandra-service.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/application/cassandra/cassandra-service.yaml)![](/images/copycode.svg "Copy application/cassandra/cassandra-service.yaml to clipboard")
```
`apiVersion: v1
kind: Service
metadata:
labels:
app: cassandra
name: cassandra
spec:
clusterIP: None
ports:
- port: 9042
selector:
app: cassandra
`
```
Create a Service to track all Cassandra StatefulSet members from the `cassandra-service.yaml` file:
```
`kubectl apply -f https://k8s.io/examples/application/cassandra/cassandra-service.yaml
`
```
### Validating (optional)
Get the Cassandra Service.
```
`kubectl get svc cassandra
`
```
The response is
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
cassandra ClusterIP None &lt;none&gt; 9042/TCP 45s
`
```
If you don't see a Service named `cassandra`, that means creation failed. Read
[Debug Services](/docs/tasks/debug/debug-application/debug-service/)
for help troubleshooting common issues.