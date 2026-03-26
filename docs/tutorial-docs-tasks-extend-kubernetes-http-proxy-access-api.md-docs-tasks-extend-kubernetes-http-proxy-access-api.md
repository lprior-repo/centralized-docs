---
id: tutorial/docs-tasks-extend-kubernetes-http-proxy-access-api.md/docs-tasks-extend-kubernetes-http-proxy-access-api
title: Use an HTTP Proxy to Access the Kubernetes API
category: tutorial
tags: ["access", "contents", "kubernetes", "proxy", "table"]
---

# Use an HTTP Proxy to Access the Kubernetes API



 > 
 > **Context**: Use an HTTP Proxy to Access the Kubernetes API | Kubernetes



### Table of Contents

* [Use an HTTP Proxy to Access the Kubernetes API](#use-an-http-proxy-to-access-the-kubernetes-api)
* [Use an HTTP Proxy to Access the Kubernetes API](#use-an-http-proxy-to-access-the-kubernetes-api)
  * [Before you begin](#before-you-begin)
  * [Using kubectl to start a proxy server](#using-kubectl-to-start-a-proxy-server)
  * [Exploring the Kubernetes API](#exploring-the-kubernetes-api)
  * [Feedback](#feedback)

---

Use an HTTP Proxy to Access the Kubernetes API | Kubernetes

## Use an HTTP Proxy to Access the Kubernetes API

## Use an HTTP Proxy to Access the Kubernetes API

This page shows how to use an HTTP proxy to access the Kubernetes API.

### Before you begin

You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:

* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
  To check the version, enter `kubectl version`.
  If you do not already have an application running in your cluster, start
  a Hello world application by entering this command:

````
`kubectl create deployment hello-app --image=gcr.io/google-samples/hello-app:2.0 --port=8080
`
````

### Using kubectl to start a proxy server

This command starts a proxy to the Kubernetes API server:

````
`kubectl proxy --port=8080
`
````

### Exploring the Kubernetes API

When the proxy server is running, you can explore the API using `curl`, `wget`,
or a browser.
Get the API versions:

````
`curl http://localhost:8080/api/
`
````

The output should look similar to this:

````
`{
"kind": "APIVersions",
"versions": [
"v1"
],
"serverAddressByClientCIDRs": [
{
"clientCIDR": "0.0.0.0/0",
"serverAddress": "10.0.2.15:8443"
}
]
}
`
````

Get a list of pods:

````
`curl http://localhost:8080/api/v1/namespaces/default/pods
`
````

The output should look similar to this:

````
`{
"kind": "PodList",
"apiVersion": "v1",
"metadata": {
"resourceVersion": "33074"
},
"items": [
{
"metadata": {
"name": "kubernetes-bootcamp-2321272333-ix8pt",
"generateName": "kubernetes-bootcamp-2321272333-",
"namespace": "default",
"uid": "ba21457c-6b1d-11e6-85f7-1ef9f1dab92b",
"resourceVersion": "33003",
"creationTimestamp": "2016-08-25T23:43:30Z",
"labels": {
"pod-template-hash": "2321272333",
"run": "kubernetes-bootcamp"
},
...
}
`
````

### Feedback

Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified June 02, 2024 at 2:43 AM PST: [Modify the image node-hello to hello-app (#46582) (d5b194da5b)](https://github.com/kubernetes/website/commit/d5b194da5b0fa6b3452384092ff8489f115972ef)

### Related Pages

* [Adding entries to Pod /etc/hosts with HostAliases](./ref-docs-tasks-network-customize-hosts-file-for-pods.md-docs-tasks-network-customize-hosts-file-for-pods.md)
* [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](./tutorial-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
* [Example: Deploying Cassandra with a StatefulSet](./tutorial-docs-tutorials-stateful-application-cassandra.md-docs-tutorials-stateful-application-cassandra.md)
* [Configure Quality of Service for Pods](./tutorial-docs-tasks-configure-pod-container-quality-service-pod.md-docs-tasks-configure-pod-container-quality-service-pod.md)
* [Configure Certificate Rotation for the Kubelet](./tutorial-docs-tasks-tls-certificate-rotation.md-docs-tasks-tls-certificate-rotation.md)
## See Also

- [Documentation Index](./COMPASS.md)
