---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 123
summary: # Example: Deploying Cassandra with a StatefulSet This tutorial shows you how to run [Apache Cassandra](https://cassandra.apache.org/) on Kubernetes. Cassandra, a database, needs persistent storage...
---

# Example: Deploying Cassandra with a StatefulSet
This tutorial shows you how to run [Apache Cassandra](https://cassandra.apache.org/) on Kubernetes.
Cassandra, a database, needs persistent storage to provide data durability (application *state*).
In this example, a custom Cassandra seed provider lets the database discover new Cassandra instances as they join the Cassandra cluster.
*StatefulSets* make it easier to deploy stateful applications into your Kubernetes cluster.
For more information on the features used in this tutorial, see
[StatefulSet](/docs/concepts/workloads/controllers/statefulset/).