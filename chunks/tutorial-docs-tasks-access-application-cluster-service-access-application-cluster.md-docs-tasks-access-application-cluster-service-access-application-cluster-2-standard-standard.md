---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 509
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