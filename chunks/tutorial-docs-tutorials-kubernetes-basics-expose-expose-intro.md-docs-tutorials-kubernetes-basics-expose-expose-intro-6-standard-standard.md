---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#6-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 451
summary: ### Step 3: Deleting a service To delete Services you can use the `delete service` subcommand. Labels can be used also here: ``` `kubectl delete service -l app=kubernetes-bootcamp ` ``` Confirm that...
---

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