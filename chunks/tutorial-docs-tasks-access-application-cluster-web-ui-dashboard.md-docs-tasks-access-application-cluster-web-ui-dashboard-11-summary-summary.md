---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#11-summary
chunk_level: summary
chunk_type: prose
heading: Accessing the Dashboard UI
token_count: 104
summary: ### Command line proxy You can enable access to the Dashboard using the `kubectl` command-line tool, by running the following command: ``` `kubectl -n kubernetes-dashboard port-forward...
---

### Command line proxy
You can enable access to the Dashboard using the `kubectl` command-line tool,
by running the following command:
```
`kubectl -n kubernetes-dashboard port-forward svc/kubernetes-dashboard-kong-proxy 8443:443
`
```
Kubectl will make Dashboard available at [https://localhost:8443](https://localhost:8443).
The UI can *only* be accessed from the machine where the command is executed. See `kubectl port-forward --help` for more options.