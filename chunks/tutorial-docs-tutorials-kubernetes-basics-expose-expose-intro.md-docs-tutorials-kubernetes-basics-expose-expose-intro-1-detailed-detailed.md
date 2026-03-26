---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#1-detailed
chunk_level: detailed
chunk_type: code
heading: Services and Labels
token_count: 760
summary: ## Services and Labels A Service routes traffic across a set of Pods. Services are the abstraction that allows pods to die and replicate in Kubernetes without impacting your application. Discovery...
---

## Services and Labels
A Service routes traffic across a set of Pods. Services are the abstraction that allows
pods to die and replicate in Kubernetes without impacting your application. Discovery
and routing among dependent Pods (such as the frontend and backend components in an application)
are handled by Kubernetes Services.
Services match a set of Pods using
[labels and selectors](/docs/concepts/overview/working-with-objects/labels/), a grouping
primitive that allows logical operation on objects in Kubernetes. Labels are key/value
pairs attached to objects and can be used in any number of ways:
* Designate objects for development, test, and production
* Embed version tags
* Classify an object using tags![](/docs/tutorials/kubernetes-basics/public/images/module_04_labels.svg)
Labels can be attached to objects at creation time or later on. They can be modified
at any time. Let's expose our application now using a Service and apply some labels.
### Step 1: Creating a new Service
Let’s verify that our application is running. We’ll use the `kubectl get` command
and look for existing Pods:
```
`kubectl get pods
`
```
If no Pods are running then it means the objects from the previous tutorials were
cleaned up. In this case, go back and recreate the deployment from the
[Using kubectl to create a Deployment](/docs/tutorials/kubernetes-basics/deploy-app/deploy-intro/#deploy-an-app)
tutorial. Please wait a couple of seconds and list the Pods again. You can continue
once you see the one Pod running.
Next, let’s list the current Services from our cluster:
```
`kubectl get services
`
```
To expose the deployment to external traffic, we'll use the kubectl expose command with the --type=NodePort option:
```
`kubectl expose deployment/kubernetes-bootcamp --type="NodePort" --port 8080
`
```
We have now a running Service called kubernetes-bootcamp. Here we see that the Service
received a unique cluster-IP, an internal port and an external-IP (the IP of the Node).
To find out what port was opened externally (for the `type: NodePort` Service) we’ll
run the `describe service` subcommand:
```
`kubectl describe services/kubernetes-bootcamp
`
```
Create an environment variable called `NODE\_PORT` that has the value of the Node
port assigned:
```
`export NODE\_PORT="$(kubectl get services/kubernetes-bootcamp -o go-template='{{(index .spec.ports 0).nodePort}}')"
echo "NODE\_PORT=$NODE\_PORT"
`
```
Now we can test that the app is exposed outside of the cluster using `curl`, the
IP address of the Node and the externally exposed port:
```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```
#### Note:
If you're running minikube with Docker Desktop as the container driver, a minikube
tunnel is needed. This is because containers inside Docker Desktop are isolated
from your host computer.
In a separate terminal window, execute:
```
`minikube service kubernetes-bootcamp --url
`
```
The output looks like this:
```
`http://127.0.0.1:51082
! Because you are using a Docker driver on darwin, the terminal needs to be open to run it.
`
```
Then use the given URL to access the app:
```
`curl 127.0.0.1:51082
`
```
And we get a response from the server. The Service is exposed.