---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#2-detailed
chunk_level: detailed
chunk_type: code
heading: Related Pages
token_count: 971
summary: ### Load Balancing Let's check that the Service is load-balancing the traffic. To find out the exposed IP and Port we can use `describe service` as we learned in the previous part of the tutorial:...
---

### Load Balancing
Let's check that the Service is load-balancing the traffic. To find out the exposed
IP and Port we can use `describe service` as we learned in the previous part of the tutorial:
```
`kubectl describe services/kubernetes-bootcamp
`
```
Create an environment variable called NODE\_PORT that has a value as the Node port:
```
`export NODE\_PORT="$(kubectl get services/kubernetes-bootcamp -o go-template='{{(index .spec.ports 0).nodePort}}')"
echo NODE\_PORT=$NODE\_PORT
`
```
Next, we’ll do a `curl` to the exposed IP address and port. Execute the command multiple times:
```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```
We hit a different Pod with every request. This demonstrates that the load-balancing is working.
The output should be similar to:
```
`Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-wp67j | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-hs9dj | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-4hjvf | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-wp67j | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-4hjvf | v=1
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
### Scale Down
To scale down the Deployment to 2 replicas, run again the `scale` subcommand:
```
`kubectl scale deployments/kubernetes-bootcamp --replicas=2
`
```
List the Deployments to check if the change was applied with the `get deployments` subcommand:
```
`kubectl get deployments
`
```
The number of replicas decreased to 2. List the number of Pods, with `get pods`:
```
`kubectl get pods -o wide
`
```
This confirms that 2 Pods were terminated.
## What's next
* Tutorial
[Performing a Rolling Update](/docs/tutorials/kubernetes-basics/update/update-intro/).
* Learn more about [ReplicaSet](/docs/concepts/workloads/controllers/replicaset/).
* Learn more about [Autoscaling](/docs/concepts/workloads/autoscaling/).
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
## Related Pages

- [deploy intro](docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md)
- [Use a Service to Access an Application in a Cluster](docs-tasks-access-application-cluster-service-access-application-cluster.md)
- [Deploy and Access the Kubernetes Dashboard](docs-tasks-access-application-cluster-web-ui-dashboard.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [expose intro](docs-tutorials-kubernetes-basics-expose-expose-intro.md)