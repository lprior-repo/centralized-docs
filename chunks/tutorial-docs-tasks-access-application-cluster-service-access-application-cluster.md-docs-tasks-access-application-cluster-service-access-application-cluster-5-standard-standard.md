---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#5-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 380
summary: ## Cleaning up To delete the Service, enter this command: ``` `kubectl delete services example-service ` ``` To delete the Deployment, the ReplicaSet, and the Pods that are running the Hello World...
---

## Cleaning up
To delete the Service, enter this command:
```
`kubectl delete services example-service
`
```
To delete the Deployment, the ReplicaSet, and the Pods that are running
the Hello World application, enter this command:
```
`kubectl delete deployment hello-world
`
```
## What's next
Follow the
[Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/)
tutorial.
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
Last modified May 28, 2024 at 9:50 AM PST: [Update node-hello image to Google's newer image (fa033cd15f)](https://github.com/kubernetes/website/commit/fa033cd15fad795b183257e59db101ba90a839ac)
## Related Pages

- [expose intro](docs-tutorials-kubernetes-basics-expose-expose-intro.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [EndpointSlices](docs-concepts-services-networking-endpoint-slices.md)
- [scale intro](docs-tutorials-kubernetes-basics-scale-scale-intro.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)