---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#14-summary
chunk_level: summary
chunk_type: prose
heading: Creating a headless Service for Cassandra
token_count: 116
summary: ## Creating a headless Service for Cassandra In Kubernetes, a [Service](/docs/concepts/services-networking/service/) describes a set of [Pods](/docs/concepts/workloads/pods/) that perform the same...
---

## Creating a headless Service for Cassandra
In Kubernetes, a [Service](/docs/concepts/services-networking/service/) describes a set of
[Pods](/docs/concepts/workloads/pods/) that perform the same task.
The following Service is used for DNS lookups between Cassandra Pods and clients within your cluster:
[`application/cassandra/cassandra-service.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/application/cassandra/cassandra-service.yaml)![](/images/copycode.svg "Copy application/cassandra/cassandra-service.yaml to clipboard")