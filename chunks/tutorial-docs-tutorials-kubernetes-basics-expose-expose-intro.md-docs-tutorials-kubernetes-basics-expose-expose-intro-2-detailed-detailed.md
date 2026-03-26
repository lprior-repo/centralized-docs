---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#2-detailed
chunk_level: detailed
chunk_type: code
heading: Related Pages
token_count: 900
summary: ### Step 2: Using labels The Deployment created automatically a label for our Pod. With the `describe deployment` subcommand you can see the name (the *key*) of that label: ``` `kubectl describe...
---

### Step 2: Using labels
The Deployment created automatically a label for our Pod. With the `describe deployment`
subcommand you can see the name (the *key*) of that label:
```
`kubectl describe deployment
`
```
Let’s use this label to query our list of Pods. We’ll use the `kubectl get pods`
command with `-l` as a parameter, followed by the label values:
```
`kubectl get pods -l app=kubernetes-bootcamp
`
```
You can do the same to list the existing Services:
```
`kubectl get services -l app=kubernetes-bootcamp
`
```
Get the name of the Pod and store it in the POD\_NAME environment variable:
```
`export POD\_NAME="$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{"\\n"}}{{end}}')"
echo "Name of the Pod: $POD\_NAME"
`
```
To apply a new label we use the label subcommand followed by the object type,
object name and the new label:
```
`kubectl label pods "$POD\_NAME" version=v1
`
```
This will apply a new label to our Pod (we pinned the application version to the Pod),
and we can check it with the `describe pod` command:
```
`kubectl describe pods "$POD\_NAME"
`
```
We see here that the label is attached now to our Pod. And we can query now the
list of pods using the new label:
```
`kubectl get pods -l version=v1
`
```
And we see the Pod.
### Step 3: Deleting a service
To delete Services you can use the `delete service` subcommand. Labels can be used
also here:
```
`kubectl delete service -l app=kubernetes-bootcamp
`
```
Confirm that the Service is gone:
```
`kubectl get services
`
```
This confirms that our Service was removed. To confirm that route is not exposed
anymore you can `curl` the previously exposed IP and port:
```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```
This proves that the application is not reachable anymore from outside of the cluster.
You can confirm that the app is still running with a `curl` from inside the pod:
```
`kubectl exec -ti $POD\_NAME -- curl http://localhost:8080
`
```
We see here that the application is up. This is because the Deployment is managing
the application. To shut down the application, you would need to delete the Deployment
as well.
## What's next
* Tutorial
[Running Multiple Instances of Your App](/docs/tutorials/kubernetes-basics/scale/scale-intro/).
* Learn more about [Service](/docs/concepts/services-networking/service/).
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

- [Use a Service to Access an Application in a Cluster](docs-tasks-access-application-cluster-service-access-application-cluster.md)
- [EndpointSlices](docs-concepts-services-networking-endpoint-slices.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [deploy intro](docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md)
- [Tools for Monitoring Resources](docs-tasks-debug-debug-cluster-resource-usage-monitoring.md)