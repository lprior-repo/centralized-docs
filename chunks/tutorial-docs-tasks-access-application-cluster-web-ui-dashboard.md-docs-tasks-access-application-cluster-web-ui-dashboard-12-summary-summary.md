---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#12-summary
chunk_level: summary
chunk_type: prose
heading: Welcome view
token_count: 124
summary: #### Note: The kubeconfig authentication method does **not** support external identity providers or X.509 certificate-based authentication. ## Welcome view When you access Dashboard on an empty...
---

#### Note:
The kubeconfig authentication method does **not** support external identity providers
or X.509 certificate-based authentication.
## Welcome view
When you access Dashboard on an empty cluster, you'll see the welcome page.
This page contains a link to this document as well as a button to deploy your first application.
In addition, you can view which system applications are running by default in the `kube-system`
[namespace](/docs/tasks/administer-cluster/namespaces/) of your cluster, for example the Dashboard itself.
![Kubernetes Dashboard welcome page](/images/docs/ui-dashboard-zerostate.png)