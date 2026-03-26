---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 259
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