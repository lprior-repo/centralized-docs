---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#3-detailed
chunk_level: detailed
chunk_type: code
heading: Related Pages
token_count: 843
summary: Make a note of the NodePort value for the Service. For example, in the preceding output, the NodePort value is 31496. 6. List the pods that are running the Hello World application: ``` `kubectl get...
---

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