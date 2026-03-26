---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#3-standard
chunk_level: standard
chunk_type: prose
heading: Creating a headless Service for Cassandra
token_count: 321
summary: ## Creating a headless Service for Cassandra In Kubernetes, a [Service](/docs/concepts/services-networking/service/) describes a set of [Pods](/docs/concepts/workloads/pods/) that perform the same...
---

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