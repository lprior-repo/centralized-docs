---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#1-standard
chunk_level: standard
chunk_type: prose
heading: Scaling an application
token_count: 316
summary: ## Scaling an application *You can create from the start a Deployment with multiple instances using the --replicas parameter for the kubectl create deployment command.* Previously we created a...
---

## Scaling an application
*You can create from the start a Deployment with multiple instances using the --replicas
parameter for the kubectl create deployment command.*
Previously we created a [Deployment](/docs/concepts/workloads/controllers/deployment/),
and then exposed it publicly via a [Service](/docs/concepts/services-networking/service/).
The Deployment created only one Pod for running our application. When traffic increases,
we will need to scale the application to keep up with user demand.
If you haven't worked through the earlier sections, start from
[Using minikube to create a cluster](/docs/tutorials/kubernetes-basics/create-cluster/cluster-intro/).
*Scaling* is accomplished by changing the number of replicas in a Deployment.
#### Note:
If you are trying this after the
[previous section](/docs/tutorials/kubernetes-basics/expose/expose-intro/), then you
may have deleted the service you created, or have created a Service of `type: NodePort`.
In this section, it is assumed that a service with `type: LoadBalancer` is created
for the kubernetes-bootcamp Deployment.
If you have *not* deleted the Service created in
[the previous section](/docs/tutorials/kubernetes-basics/expose/expose-intro/),
first delete that Service and then run the following command to create a new Service
with its `type` set to `LoadBalancer`:
```
`kubectl expose deployment/kubernetes-bootcamp --type="LoadBalancer" --port 8080
`
```