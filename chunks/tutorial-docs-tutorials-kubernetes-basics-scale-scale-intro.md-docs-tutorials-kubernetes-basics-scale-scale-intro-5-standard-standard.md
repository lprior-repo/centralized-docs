---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#5-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 485
summary: ### Scale Down To scale down the Deployment to 2 replicas, run again the `scale` subcommand: ``` `kubectl scale deployments/kubernetes-bootcamp --replicas=2 ` ``` List the Deployments to check if the...
---

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