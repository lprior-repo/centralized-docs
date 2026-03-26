---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#2-detailed
chunk_level: detailed
chunk_type: code
heading: Before you begin
token_count: 870
summary: ## Before you begin You need to have a Kubernetes cluster, and the kubectl command-line tool must be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster...
---

## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)## Objectives
* Run two instances of a Hello World application.
* Create a Service object that exposes a node port.
* Use the Service object to access the running application.## Creating a service for an application running in two pods
Here is the configuration file for the application Deployment:
[`service/access/hello-application.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/access/hello-application.yaml)![](/images/copycode.svg "Copy service/access/hello-application.yaml to clipboard")
```
`apiVersion: apps/v1
kind: Deployment
metadata:
name: hello-world
spec:
selector:
matchLabels:
run: load-balancer-example
replicas: 2
template:
metadata:
labels:
run: load-balancer-example
spec:
containers:
- name: hello-world
image: us-docker.pkg.dev/google-samples/containers/gke/hello-app:2.0
ports:
- containerPort: 8080
protocol: TCP
`
```
1. Run a Hello World application in your cluster:
Create the application Deployment using the file above:
```
`kubectl apply -f https://k8s.io/examples/service/access/hello-application.yaml
`
```
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