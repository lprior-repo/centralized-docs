---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 353
summary: # Deploy and Access the Kubernetes Dashboard Deploy the web UI (Kubernetes Dashboard) and access it. **Kubernetes Dashboard is deprecated and unmaintained.** The Kubernetes Dashboard project has been...
---

# Deploy and Access the Kubernetes Dashboard
Deploy the web UI (Kubernetes Dashboard) and access it.
**Kubernetes Dashboard is deprecated and unmaintained.**
The Kubernetes Dashboard project has been archived and is no longer actively maintained.
For new installations, consider using [Headlamp](https://headlamp.dev/).
#### Note:
For in-cluster deployments similar to Kubernetes Dashboard, see the
[Headlamp in-cluster installation guide](https://headlamp.dev/docs/latest/installation/in-cluster/).
Dashboard is a web-based Kubernetes user interface.
You can use Dashboard to deploy containerized applications to a Kubernetes cluster,
troubleshoot your containerized application, and manage the cluster resources.
You can use Dashboard to get an overview of applications running on your cluster,
as well as for creating or modifying individual Kubernetes resources
(such as Deployments, Jobs, DaemonSets, etc).
For example, you can scale a Deployment, initiate a rolling update, restart a pod
or deploy new applications using a deploy wizard.
Dashboard also provides information on the state of Kubernetes resources in your cluster and on any errors that may have occurred.
![Kubernetes Dashboard UI](/images/docs/ui-dashboard.png)
#### Note:
Kubernetes Dashboard supports only Helm-based installation currently as it is faster
and gives us better control over all dependencies required by Dashboard to run.
The Dashboard UI is not deployed by default. To deploy it, run the following command:
```
`# Add kubernetes-dashboard repository
helm repo add kubernetes-dashboard https://kubernetes.github.io/dashboard/
# Deploy a Helm Release named "kubernetes-dashboard" using the kubernetes-dashboard chart
helm upgrade --install kubernetes-dashboard kubernetes-dashboard/kubernetes-dashboard --create-namespace --namespace kubernetes-dashboard
`
```