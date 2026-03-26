---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#4-standard
chunk_level: standard
chunk_type: code
heading: What's next
token_count: 509
summary: ``` `kubectl get pods --selector=\"run=load-balancer-example\" --output=wide ` ``` The output is similar to this: ``` `NAME READY STATUS ... IP NODE hello-world-2895499144-bsbk5 1/1 Running ......
---

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