---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#4-standard
chunk_level: standard
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 502
summary: ### View the app [Pods](/docs/concepts/workloads/pods/) that are running inside Kubernetes are running on a private, isolated network. By default they are visible from other pods and services within...
---

### View the app
[Pods](/docs/concepts/workloads/pods/) that are running inside Kubernetes are running
on a private, isolated network. By default they are visible from other pods and services
within the same Kubernetes cluster, but not outside that network. When we use `kubectl`,
we're interacting through an API endpoint to communicate with our application.
We will cover other options on how to expose your application outside the Kubernetes
cluster later, in [Module 4](/docs/tutorials/kubernetes-basics/expose/).
Also as a basic tutorial, we're not explaining what `Pods` are in any
detail here, it will be covered in later topics.
The `kubectl proxy` command can create a proxy that will forward communications
into the cluster-wide, private network. The proxy can be terminated by pressing
control-C and won't show any output while it's running.
**You need to open a second terminal window to run the proxy.**
```
`kubectl proxy
`
```
We now have a connection between our host (the terminal) and the Kubernetes cluster.
The proxy enables direct access to the API from these terminals.
You can see all those APIs hosted through the proxy endpoint. For example, we can
query the version directly through the API using the `curl` command:
```
`curl http://localhost:8001/version
`
```
#### Note:
If port 8001 is not accessible, ensure that the `kubectl proxy` that you started
above is running in the second terminal.
The API server will automatically create an endpoint for each pod, based on the
pod name, that is also accessible through the proxy.
First we need to get the Pod name, and we'll store it in the environment variable `POD\_NAME`.
```
`export POD\_NAME=$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{"\\n"}}{{end}}')
echo Name of the Pod: $POD\_NAME
`
```
You can access the Pod through the proxied API, by running:
```
`curl http://localhost:8001/api/v1/namespaces/default/pods/$POD\_NAME:8080/proxy/
`
```
In order for the new Deployment to be accessible without using the proxy, a Service
is required which will be explained in [Module 4](/docs/tutorials/kubernetes-basics/expose/).