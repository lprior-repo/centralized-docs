---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#2-standard
chunk_level: standard
chunk_type: prose
heading: Welcome view
token_count: 326
summary: ## Accessing the Dashboard UI To protect your cluster data, Dashboard deploys with a minimal RBAC configuration by default. Currently, Dashboard only supports logging in with a Bearer Token. To...
---

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