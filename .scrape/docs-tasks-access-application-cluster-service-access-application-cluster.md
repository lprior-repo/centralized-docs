---
url: https://kubernetes.io/docs/tasks/access-application-cluster/service-access-application-cluster/
title: Use a Service to Access an Application in a Cluster
word_count: 778
filtered: true
elements_removed: 0
density_score: 0.87
---

## Table of Contents

- [Use a Service to Access an Application in a Cluster](#use-a-service-to-access-an-application-in-a-cluster)
- [Use a Service to Access an Application in a Cluster](#use-a-service-to-access-an-application-in-a-cluster)
  - [Before you begin](#before-you-begin)
  - [Using a service configuration file](#using-a-service-configuration-file)
  - [Cleaning up](#cleaning-up)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

Use a Service to Access an Application in a Cluster | Kubernetes
# Use a Service to Access an Application in a Cluster
# Use a Service to Access an Application in a Cluster
This page shows how to create a Kubernetes Service object that external
clients can use to access an application running in a cluster. The Service
provides load balancing for an application that has two running instances.
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
7. Get the public IP address of one of your nodes that is running
a Hello World pod. How you get this address depends on how you set
up your cluster. For example, if you are using Minikube, you can
see the node address by running `kubectl cluster-info`. If you are
using Google Compute Engine instances, you can use the
`gcloud compute instances list` command to see the public addresses of your
nodes.
8. On your chosen node, create a firewall rule that allows TCP traffic
on your node port. For example, if your Service has a NodePort value of
31568, create a firewall rule that allows TCP traffic on port 31568. Different
cloud providers offer different ways of configuring firewall rules.
9. Use the node address and node port to access the Hello World application:
```
`curl http://&lt;public-node-ip&gt;:&lt;node-port&gt;
`
```
where `&lt;public-node-ip&gt;` is the public IP address of your node,
and `&lt;node-port&gt;` is the NodePort value for your service. The
response to a successful request is a hello message:
```
`Hello, world!
Version: 2.0.0
Hostname: hello-world-cdd4458f4-m47c8
`
```
## Using a service configuration file
As an alternative to using `kubectl expose`, you can use a
[service configuration file](/docs/concepts/services-networking/service/)
to create a Service.
## Cleaning up
To delete the Service, enter this command:
```
`kubectl delete services example-service
`
```
To delete the Deployment, the ReplicaSet, and the Pods that are running
the Hello World application, enter this command:
```
`kubectl delete deployment hello-world
`
```
## What's next
Follow the
[Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/)
tutorial.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified May 28, 2024 at 9:50 AM PST: [Update node-hello image to Google's newer image (fa033cd15f)](https://github.com/kubernetes/website/commit/fa033cd15fad795b183257e59db101ba90a839ac)
## Related Pages

- [expose intro](docs-tutorials-kubernetes-basics-expose-expose-intro.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [EndpointSlices](docs-concepts-services-networking-endpoint-slices.md)
- [scale intro](docs-tutorials-kubernetes-basics-scale-scale-intro.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
