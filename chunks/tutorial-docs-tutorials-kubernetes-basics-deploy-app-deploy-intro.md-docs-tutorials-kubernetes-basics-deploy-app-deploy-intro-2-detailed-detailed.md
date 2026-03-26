---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#2-detailed
chunk_level: detailed
chunk_type: code
heading: Feedback
token_count: 938
summary: ### Deploy an app Let’s deploy our first app on Kubernetes with the `kubectl create deployment` command. We need to provide the deployment name and app image location (include the full repository url...
---

### Deploy an app
Let’s deploy our first app on Kubernetes with the `kubectl create deployment` command.
We need to provide the deployment name and app image location (include the full
repository url for images hosted outside Docker Hub).
```
`kubectl create deployment kubernetes-bootcamp --image=gcr.io/google-samples/kubernetes-bootcamp:v1
`
```
Great! You just deployed your first application by creating a deployment. This performed a few things for you:
* searched for a suitable node where an instance of the application could be run (we have only 1 available node)
* scheduled the application to run on that Node
* configured the cluster to reschedule the instance on a new Node when needed
To list your deployments use the `kubectl get deployments` command:
```
`kubectl get deployments
`
```
We see that there is 1 deployment running a single instance of your app. The instance
is running inside a container on your node.
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
## What's next
* Tutorial [Viewing Pods and Nodes](/docs/tutorials/kubernetes-basics/explore/explore-intro/).
* Learn more about [Deployments](/docs/concepts/workloads/controllers/deployment/).
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
Last modified March 12, 2026 at 3:33 PM PST: [Clarify POSIX shell wording in prerequisites (5d98744874)](https://github.com/kubernetes/website/commit/5d987448741d04c26fd0edf531e08594d2869e80)