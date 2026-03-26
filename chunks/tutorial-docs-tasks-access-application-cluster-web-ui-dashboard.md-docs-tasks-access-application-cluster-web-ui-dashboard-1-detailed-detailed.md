---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Welcome view
token_count: 680
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
## Accessing the Dashboard UI
To protect your cluster data, Dashboard deploys with a minimal RBAC configuration by default.
Currently, Dashboard only supports logging in with a Bearer Token.
To create a token for this demo, you can follow our guide on
[creating a sample user](https://github.com/kubernetes/dashboard/blob/master/docs/user/access-control/creating-sample-user.md).
#### Warning:
The sample user created in the tutorial will have administrative privileges and is for educational purposes only.
### Command line proxy
You can enable access to the Dashboard using the `kubectl` command-line tool,
by running the following command:
```
`kubectl -n kubernetes-dashboard port-forward svc/kubernetes-dashboard-kong-proxy 8443:443
`
```
Kubectl will make Dashboard available at [https://localhost:8443](https://localhost:8443).
The UI can *only* be accessed from the machine where the command is executed. See `kubectl port-forward --help` for more options.
#### Note:
The kubeconfig authentication method does **not** support external identity providers
or X.509 certificate-based authentication.
## Welcome view
When you access Dashboard on an empty cluster, you'll see the welcome page.
This page contains a link to this document as well as a button to deploy your first application.
In addition, you can view which system applications are running by default in the `kube-system`
[namespace](/docs/tasks/administer-cluster/namespaces/) of your cluster, for example the Dashboard itself.
![Kubernetes Dashboard welcome page](/images/docs/ui-dashboard-zerostate.png)