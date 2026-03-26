---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#4-standard
chunk_level: standard
chunk_type: code
heading: Services and Labels
token_count: 404
summary: ### Step 1: Creating a new Service Let’s verify that our application is running. We’ll use the `kubectl get` command and look for existing Pods: ``` `kubectl get pods ` ``` If no Pods are running...
---

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