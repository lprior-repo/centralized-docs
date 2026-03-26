---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Overview of Kubernetes Services
token_count: 981
summary: ## Table of Contents    - [Objectives](#objectives)   - [Overview of Kubernetes Services](#overview-of-kubernetes-services)   - [Services and Labels](#services-and-labels)     - [Step 1: Creating a...
---

## Table of Contents

  - [Objectives](#objectives)
  - [Overview of Kubernetes Services](#overview-of-kubernetes-services)
  - [Services and Labels](#services-and-labels)
    - [Step 1: Creating a new Service](#step-1-creating-a-new-service)
      - [Note:](#note)
    - [Step 2: Using labels](#step-2-using-labels)
    - [Step 3: Deleting a service](#step-3-deleting-a-service)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

## Objectives
* Learn about a Service in Kubernetes.
* Understand how labels and selectors relate to a Service.
* Expose an application outside a Kubernetes cluster.## Before you begin
The shell commands in this tutorial use POSIX shell syntax, which is supported by
the default shells on most Linux and macOS systems (for example, bash, zsh, or sh).
Windows users must use a POSIX-compatible shell such as
[Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)
or [Git Bash](https://gitforwindows.org/) to run the commands as written.
Commands that use `export`, `$()`, and similar constructs are **not** compatible
with PowerShell or the Windows Command Prompt.
## Overview of Kubernetes Services
Kubernetes [Pods](/docs/concepts/workloads/pods/) are mortal. Pods have a
[lifecycle](/docs/concepts/workloads/pods/pod-lifecycle/). When a worker node dies,
the Pods running on the Node are also lost. A [Replicaset](/docs/concepts/workloads/controllers/replicaset/)
might then dynamically drive the cluster back to the desired state via the creation
of new Pods to keep your application running. As another example, consider an image-processing
backend with 3 replicas. Those replicas are exchangeable; the front-end system should
not care about backend replicas or even if a Pod is lost and recreated. That said,
each Pod in a Kubernetes cluster has a unique IP address, even Pods on the same Node,
so there needs to be a way of automatically reconciling changes among Pods so that your
applications continue to function.
*A Kubernetes Service is an abstraction layer which defines a logical set of Pods and
enables external traffic exposure, load balancing and service discovery for those Pods.*
A [Service](/docs/concepts/services-networking/service/) in Kubernetes is an abstraction
which defines a logical set of Pods and a policy by which to access them. Services
enable a loose coupling between dependent Pods. A Service is defined using YAML or JSON,
like all Kubernetes object manifests. The set of Pods targeted by a Service is usually
determined by a *label selector* (see below for why you might want a Service without
including a `selector` in the spec).
Although each Pod has a unique IP address, those IPs are not exposed outside the
cluster without a Service. Services allow your applications to receive traffic.
Services can be exposed in different ways by specifying a `type` in the `spec` of the Service:
* *ClusterIP* (default) - Exposes the Service on an internal IP in the cluster. This
type makes the Service only reachable from within the cluster.
* *NodePort* - Exposes the Service on the same port of each selected Node in the cluster using NAT.
Makes a Service accessible from outside the cluster using `NodeIP:NodePort`. Superset of ClusterIP.
* *LoadBalancer* - Creates an external load balancer in the current cloud (if supported)
and assigns a fixed, external IP to the Service. Superset of NodePort.
* *ExternalName* - Maps the Service to the contents of the `externalName` field
(e.g. `foo.bar.example.com`), by returning a `CNAME` record with its value.
No proxying of any kind is set up. This type requires v1.7 or higher of `kube-dns`,
or CoreDNS version 0.0.8 or higher.
More information about the different types of Services can be found in the
[Using Source IP](/docs/tutorials/services/source-ip/) tutorial. Also see
[Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/).
Additionally, note that there are some use cases with Services that involve not defining
a `selector` in the spec. A Service created without `selector` will also not create
the corresponding Endpoints object. This allows users to manually map a Service to
specific endpoints. Another possibility why there may be no selector is you are strictly
using `type: ExternalName`.