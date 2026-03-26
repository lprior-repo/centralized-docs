---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#3-standard
chunk_level: standard
chunk_type: code
heading: Before you begin
token_count: 455
summary: The preceding command creates a [Deployment](/docs/concepts/workloads/controllers/deployment/) and an associated [ReplicaSet](/docs/concepts/workloads/controllers/replicaset/). The ReplicaSet has two...
---

The preceding command creates a
[Deployment](/docs/concepts/workloads/controllers/deployment/)
and an associated
[ReplicaSet](/docs/concepts/workloads/controllers/replicaset/).
The ReplicaSet has two
[Pods](/docs/concepts/workloads/pods/)
each of which runs the Hello World application.
2. Display information about the Deployment:
```
`kubectl get deployments hello-world
kubectl describe deployments hello-world
`
```
3. Display information about your ReplicaSet objects:
```
`kubectl get replicasets
kubectl describe replicasets
`
```
4. Create a Service object that exposes the deployment:
```
`kubectl expose deployment hello-world --type=NodePort --name=example-service
`
```
5. Display information about the Service:
```
`kubectl describe services example-service
`
```
The output is similar to this:
```
`Name: example-service
Namespace: default
Labels: run=load-balancer-example
Annotations: &lt;none&gt;
Selector: run=load-balancer-example
Type: NodePort
IP: 10.32.0.16
Port: &lt;unset&gt; 8080/TCP
TargetPort: 8080/TCP
NodePort: &lt;unset&gt; 31496/TCP
Endpoints: 10.200.1.4:8080,10.200.2.5:8080
Session Affinity: None
Events: &lt;none&gt;
`
```
Make a note of the NodePort value for the Service. For example,
in the preceding output, the NodePort value is 31496.
6. List the pods that are running the Hello World application:
```
`kubectl get pods --selector="run=load-balancer-example" --output=wide
`
```
The output is similar to this:
```
`NAME READY STATUS ... IP NODE
hello-world-2895499144-bsbk5 1/1 Running ... 10.200.1.4 worker1
hello-world-2895499144-m1pwt 1/1 Running ... 10.200.2.5 worker2
`
```