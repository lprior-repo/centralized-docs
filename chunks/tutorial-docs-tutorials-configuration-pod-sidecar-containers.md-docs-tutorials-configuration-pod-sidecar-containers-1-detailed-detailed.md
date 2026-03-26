---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Benefits of a built-in sidecar container
token_count: 927
summary: # Adopting Sidecar Containers This section is relevant for people adopting a new built-in [sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/) feature for their workloads. Sidecar...
---

# Adopting Sidecar Containers
This section is relevant for people adopting a new built-in
[sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/) feature for their workloads.
Sidecar container is not a new concept as posted in the
[blog post](/blog/2015/06/the-distributed-system-toolkit-patterns/).
Kubernetes allows running multiple containers in a Pod to implement this concept.
However, running a sidecar container as a regular container
has a lot of limitations being fixed with the new built-in sidecar containers support.
FEATURE STATE:
`Kubernetes v1.33 [stable]`(enabled by default)
## Objectives
* Understand the need for sidecar containers
* Be able to troubleshoot issues with the sidecar containers
* Understand options to universally "inject" sidecar containers to any workload## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)Your Kubernetes server must be at or later than version 1.29.
To check the version, enter `kubectl version`.
## Sidecar containers overview
Sidecar containers are secondary containers that run along with the main
application container within the same [Pod](/docs/concepts/workloads/pods/).
These containers are used to enhance or to extend the functionality of the primary *app
container* by providing additional services, or functionalities such as logging, monitoring,
security, or data synchronization, without directly altering the primary application code.
You can read more in the [Sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/)
concept page.
The concept of sidecar containers is not new and there are multiple implementations of this concept.
As well as sidecar containers that you, the person defining the Pod, want to run, you can also find
that some [addons](/docs/concepts/cluster-administration/addons/) modify Pods - before the Pods
start running - so that there are extra sidecar containers. The mechanisms to *inject* those extra
sidecars are often [mutating webhooks](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook).
For example, a service mesh addon might inject a sidecar that configures mutual TLS and encryption
in transit between different Pods.
While the concept of sidecar containers is not new,
the native implementation of this feature in Kubernetes, however, is new. And as with every new feature,
adopting this feature may present certain challenges.
This tutorial explores challenges and solutions that can be experienced by end users as well as
by authors of sidecar containers.
## Benefits of a built-in sidecar container
Using Kubernetes' native support for sidecar containers provides several benefits:
1. You can configure a native sidecar container to start ahead of
[init containers](/docs/concepts/workloads/pods/init-containers/).
2. The built-in sidecar containers can be authored to guarantee that they are terminated last.
Sidecar containers are terminated with a `SIGTERM` signal once all the regular containers
are completed and terminated. If the sidecar container isn’t gracefully shut down, a
`SIGKILL` signal will be used to terminate it.
3. With Jobs, when Pod's `restartPolicy: OnFailure` or `restartPolicy: Never`,
native sidecar containers do not block Pod completion. With legacy sidecar containers,
special care is needed to handle this situation.
4. Also, with Jobs, built-in sidecar containers would keep being restarted once they are done,
even if regular containers would not with Pod's `restartPolicy: Never`.
See [differences from init containers](/docs/concepts/workloads/pods/sidecar-containers/#differences-from-application-containers)
to learn more about it.