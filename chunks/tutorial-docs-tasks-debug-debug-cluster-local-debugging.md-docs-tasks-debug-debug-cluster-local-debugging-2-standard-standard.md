---
doc_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging
chunk_id: tutorial/docs-tasks-debug-debug-cluster-local-debugging.md/docs-tasks-debug-debug-cluster-local-debugging#2-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 429
summary: ## Developing or debugging an existing service When developing an application on Kubernetes, you typically program or debug a single service. The service might require access to other services for...
---

## Developing or debugging an existing service
When developing an application on Kubernetes, you typically program
or debug a single service. The service might require access to other
services for testing and debugging. One option is to use the continuous
deployment pipeline, but even the fastest deployment pipeline introduces
a delay in the program or debug cycle.
Use the `telepresence intercept $SERVICE\_NAME --port $LOCAL\_PORT:$REMOTE\_PORT`
command to create an "intercept" for rerouting remote service traffic.
Where:
* `$SERVICE\_NAME` is the name of your local service
* `$LOCAL\_PORT` is the port that your service is running on your local workstation
* And `$REMOTE\_PORT` is the port your service listens to in the cluster
Running this command tells Telepresence to send remote traffic to your
local service instead of the service in the remote Kubernetes cluster.
Make edits to your service source code locally, save, and see the corresponding
changes when accessing your remote application take effect immediately.
You can also run your local service using a debugger or any other local development tool.
## How does Telepresence work?
Telepresence installs a traffic-agent sidecar next to your existing
application's container running in the remote cluster. It then captures
all traffic requests going into the Pod, and instead of forwarding this
to the application in the remote cluster, it routes all traffic (when you
create a [global intercept](https://www.getambassador.io/docs/telepresence/latest/concepts/intercepts/#global-intercept)
or a subset of the traffic (when you create a
[personal intercept](https://www.getambassador.io/docs/telepresence/latest/concepts/intercepts/#personal-intercept))
to your local development environment.
## What's next
If you're interested in a hands-on tutorial, check out
[this tutorial](https://cloud.google.com/community/tutorials/developing-services-with-k8s)
that walks through locally developing the Guestbook application on Google Kubernetes Engine.
For further reading, visit the [Telepresence website](https://www.telepresence.io).