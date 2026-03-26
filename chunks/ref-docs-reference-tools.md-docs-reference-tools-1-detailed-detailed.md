---
doc_id: ref/docs-reference-tools.md/docs-reference-tools
chunk_id: ref/docs-reference-tools.md/docs-reference-tools#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 986
summary: # Other Tools Kubernetes contains several tools to help you work with the Kubernetes system. ## crictl [`crictl`](https://github.com/kubernetes-sigs/cri-tools) is a command-line interface for...
---

# Other Tools
Kubernetes contains several tools to help you work with the Kubernetes system.
## crictl
[`crictl`](https://github.com/kubernetes-sigs/cri-tools) is a command-line
interface for inspecting and debugging [CRI](/docs/concepts/architecture/cri)-compatible
container runtimes.
## Dashboard
[`Dashboard`](/docs/tasks/access-application-cluster/web-ui-dashboard/), the web-based user interface of Kubernetes, allows you to deploy containerized applications
to a Kubernetes cluster, troubleshoot them, and manage the cluster and its
resources itself.
## Headlamp
[Headlamp](https://headlamp.dev/) is an extensible Kubernetes graphical user
interface, and is an optional Kubernetes cluster component.
Headlamp is part of the Kubernetes project.
Headlamp provides:
* A modern, user-friendly graphical interface for cluster management and troubleshooting
* Support for both in-cluster deployment and desktop application modes
* Extensibility through a plugin system
* RBAC-based controls that adapt to user permissions## Helm
🛇 This item links to a third party project or product that is not part of Kubernetes itself. [More information](#third-party-content-disclaimer)
[Helm](https://helm.sh/) is a tool for managing packages of pre-configured
Kubernetes resources. These packages are known as *Helm charts*.
Use Helm to:
* Find and use popular software packaged as Kubernetes charts
* Share your own applications as Kubernetes charts
* Create reproducible builds of your Kubernetes applications
* Intelligently manage your Kubernetes manifest files
* Manage releases of Helm packages## Kompose
[`Kompose`](https://github.com/kubernetes/kompose) is a tool to help Docker Compose users move to Kubernetes.
Use Kompose to:
* Translate a Docker Compose file into Kubernetes objects
* Go from local Docker development to managing your application via Kubernetes
* Convert v1 or v2 Docker Compose `yaml` files or [Distributed Application Bundles](https://docs.docker.com/compose/bundles/)## Kui
[`Kui`](https://github.com/kubernetes-sigs/kui) is a GUI tool that takes your normal `kubectl` command line requests and responds with graphics.
Instead of ASCII tables, Kui provides a GUI rendering with tables that you can sort.
Kui lets you:
* Directly click on long, auto-generated resource names instead of copying and pasting
* Type in `kubectl` commands and see them execute, even sometimes faster than `kubectl` itself
* Query a [Job](/docs/concepts/workloads/controllers/job/) and see its execution rendered
as a waterfall diagram
* Click through resources in your cluster using a tabbed UI## Minikube
[`minikube`](https://minikube.sigs.k8s.io/docs/) is a tool that
runs a single-node Kubernetes cluster locally on your workstation for
development and testing purposes.
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
Last modified February 08, 2026 at 9:10 PM PST: [chore: remove trailing space (981e969757)](https://github.com/kubernetes/website/commit/981e969757c20d9accaadc4c7675342801a07856)
Items on this page refer to third party products or projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for those third-party products or projects. See the [CNCF website guidelines](https://github.com/cncf/foundation/blob/main/policies-guidance/website-guidelines.md) for more details.
You should read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before proposing a change that adds an extra third-party link.
## Related Pages

- [Metrics for Kubernetes Object States](docs-concepts-cluster-administration-kube-state-metrics.md)
- [Developing and debugging services locally using telepresence](docs-tasks-debug-debug-cluster-local-debugging.md)
- [Service Accounts](docs-concepts-security-service-accounts.md)
- [Pod Security Standards](docs-concepts-security-pod-security-standards.md)
- [Observability](docs-concepts-cluster-administration-observability.md)