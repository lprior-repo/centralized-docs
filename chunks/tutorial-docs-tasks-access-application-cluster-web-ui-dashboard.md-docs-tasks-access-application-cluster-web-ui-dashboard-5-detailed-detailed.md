---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 681
summary: ## Using Dashboard Following sections describe views of the Kubernetes Dashboard UI; what they provide and how can they be used. #### Admin overview For cluster and namespace administrators,...
---

## Using Dashboard
Following sections describe views of the Kubernetes Dashboard UI; what they provide and how can they be used.
#### Admin overview
For cluster and namespace administrators, Dashboard lists Nodes, Namespaces and PersistentVolumes and has detail views for them.
Node list view contains CPU and memory usage metrics aggregated across all Nodes.
The details view shows the metrics for a Node, its specification, status,
allocated resources, events and pods running on the node.
#### Workloads
Shows all applications running in the selected namespace.
The view lists applications by workload kind (for example: Deployments, ReplicaSets, StatefulSets).
Each workload kind can be viewed separately.
The lists summarize actionable information about the workloads,
such as the number of ready pods for a ReplicaSet or current memory usage for a Pod.
Detail views for workloads show status and specification information and
surface relationships between objects.
For example, Pods that ReplicaSet is controlling or new ReplicaSets and HorizontalPodAutoscalers for Deployments.
#### Services
Shows Kubernetes resources that allow for exposing services to external world and
discovering them within a cluster.
For that reason, Service and Ingress views show Pods targeted by them,
internal endpoints for cluster connections and external endpoints for external users.
#### Storage
Storage view shows PersistentVolumeClaim resources which are used by applications for storing data.
#### ConfigMaps and Secrets
Shows all Kubernetes resources that are used for live configuration of applications running in clusters.
The view allows for editing and managing config objects and displays secrets hidden by default.
#### Logs viewer
Pod lists and detail pages link to a logs viewer that is built into Dashboard.
The viewer allows for drilling down logs from containers belonging to a single Pod.
![Logs viewer](/images/docs/ui-dashboard-logs-view.png)
## What's next
For more information, see the
[Kubernetes Dashboard project page](https://github.com/kubernetes/dashboard).
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
Last modified February 03, 2026 at 5:52 PM PST: [Use a pageinfo rather than a warning callout. (3e0aa51ca3)](https://github.com/kubernetes/website/commit/3e0aa51ca3ead12bf278b8c590e6689bef724811)
## Related Pages

- [Use a Service to Access an Application in a Cluster](docs-tasks-access-application-cluster-service-access-application-cluster.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [EndpointSlices](docs-concepts-services-networking-endpoint-slices.md)
- [scale intro](docs-tutorials-kubernetes-basics-scale-scale-intro.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)