---
url: https://kubernetes.io/docs/tasks/debug/debug-cluster/local-debugging/
title: Developing and debugging services locally using telepresence
word_count: 638
filtered: true
elements_removed: 0
density_score: 0.89
---

## Table of Contents

- [Developing and debugging services locally using telepresence](#developing-and-debugging-services-locally-using-telepresence)
  - [Before you begin](#before-you-begin)
  - [Developing or debugging an existing service](#developing-or-debugging-an-existing-service)
  - [How does Telepresence work?](#how-does-telepresence-work)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

# Developing and debugging services locally using telepresence
**Note:** This section links to third party projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for these projects, which are listed alphabetically. To add a project to this list, read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before submitting a change. [More information.](#third-party-content-disclaimer)
Kubernetes applications usually consist of multiple, separate services,
each running in its own container. Developing and debugging these services
on a remote Kubernetes cluster can be cumbersome, requiring you to
[get a shell on a running container](/docs/tasks/debug/debug-application/get-shell-running-container/)
in order to run debugging tools.
`telepresence` is a tool to ease the process of developing and debugging
services locally while proxying the service to a remote Kubernetes cluster.
Using `telepresence` allows you to use custom tools, such as a debugger and
IDE, for a local service and provides the service full access to ConfigMap,
secrets, and the services running on the remote cluster.
This document describes using `telepresence` to develop and debug services
running on a remote cluster locally.
## Before you begin
* Kubernetes cluster is installed
* `kubectl` is configured to communicate with the cluster
* [Telepresence](https://www.telepresence.io/docs/latest/quick-start/) is installed## Connecting your local machine to a remote Kubernetes cluster
After installing `telepresence`, run `telepresence connect` to launch
its Daemon and connect your local workstation to the cluster.
```
`$ telepresence connect
Launching Telepresence Daemon
...
Connected to context default (https://&lt;cluster public IP&gt;)
`
```
You can curl services using the Kubernetes syntax e.g. `curl -ik https://kubernetes.default`
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
Last modified November 24, 2023 at 4:55 PM PST: [Solves issue: #44034 (802dde6897)](https://github.com/kubernetes/website/commit/802dde68970ba9cc359c231b7c131c41c069c53c)
Items on this page refer to third party products or projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for those third-party products or projects. See the [CNCF website guidelines](https://github.com/cncf/foundation/blob/main/policies-guidance/website-guidelines.md) for more details.
You should read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before proposing a change that adds an extra third-party link.
## Related Pages

- [Other Tools](docs-reference-tools.md)
- [Metrics for Kubernetes Object States](docs-concepts-cluster-administration-kube-state-metrics.md)
- [Service Accounts](docs-concepts-security-service-accounts.md)
- [Pod Security Standards](docs-concepts-security-pod-security-standards.md)
- [Tools for Monitoring Resources](docs-tasks-debug-debug-cluster-resource-usage-monitoring.md)
