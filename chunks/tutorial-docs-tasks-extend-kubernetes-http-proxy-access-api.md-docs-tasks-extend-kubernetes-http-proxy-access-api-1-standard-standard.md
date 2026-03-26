---
doc_id: tutorial/docs-tasks-extend-kubernetes-http-proxy-access-api.md/docs-tasks-extend-kubernetes-http-proxy-access-api
chunk_id: tutorial/docs-tasks-extend-kubernetes-http-proxy-access-api.md/docs-tasks-extend-kubernetes-http-proxy-access-api#1-standard
chunk_level: standard
chunk_type: prose
heading: Using kubectl to start a proxy server
token_count: 290
summary: # Use an HTTP Proxy to Access the Kubernetes API This page shows how to use an HTTP proxy to access the Kubernetes API. ## Before you begin You need to have a Kubernetes cluster, and the kubectl...
---

# Use an HTTP Proxy to Access the Kubernetes API
This page shows how to use an HTTP proxy to access the Kubernetes API.
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
To check the version, enter `kubectl version`.
If you do not already have an application running in your cluster, start
a Hello world application by entering this command:
```
`kubectl create deployment hello-app --image=gcr.io/google-samples/hello-app:2.0 --port=8080
`
```
## Using kubectl to start a proxy server
This command starts a proxy to the Kubernetes API server:
```
`kubectl proxy --port=8080
`
```