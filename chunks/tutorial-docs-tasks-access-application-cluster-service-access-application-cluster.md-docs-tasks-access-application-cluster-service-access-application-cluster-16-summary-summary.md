---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#16-summary
chunk_level: summary
chunk_type: prose
heading: What's next
token_count: 128
summary: ## Using a service configuration file As an alternative to using `kubectl expose`, you can use a [service configuration file](/docs/concepts/services-networking/service/) to create a Service. ##...
---

## Using a service configuration file
As an alternative to using `kubectl expose`, you can use a
[service configuration file](/docs/concepts/services-networking/service/)
to create a Service.
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