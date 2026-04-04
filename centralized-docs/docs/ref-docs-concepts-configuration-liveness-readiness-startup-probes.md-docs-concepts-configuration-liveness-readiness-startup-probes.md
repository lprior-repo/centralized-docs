---
id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes
title: Liveness, Readiness, and Startup Probes
category: ref
tags: ["contents", "liveness,", "probe", "probes", "readiness"]
---

## Table of Contents

* [Liveness, Readiness, and Startup Probes](#liveness-readiness-and-startup-probes)
  * [Readiness probe](#readiness-probe)
  * [Feedback](#feedback)

---

# Liveness, Readiness, and Startup Probes



 > 
 > **Context**: Kubernetes lets you define  probes  to continuously monitor the health of containers in a Pod. Based on probe results, Kubernetes can restart unhealth



Kubernetes lets you define *probes* to continuously monitor the health of containers in a Pod.
Based on probe results, Kubernetes can restart unhealthy containers or stop sending traffic to containers that are not ready.
There are three types of probes, each serving a different purpose:

* [Startup probe](#startup-probe)
* [Liveness probe](#liveness-probe)
* [Readiness probe](#readiness-probe)\## Startup probe
  Startup probes verify whether the application within a container is started. If a startup probe is configured,
  Kubernetes does not execute liveness or readiness probes until the startup probe succeeds, allowing the application time to finish its initialization.
  This type of probe is only executed at startup, unlike liveness and readiness probes, which are run periodically.
* Read more about the [Configure Liveness, Readiness and Startup Probes](/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/).## Liveness probe
  Liveness probes determine when to restart a container. For example, liveness probes could catch a deadlock when an application is running but unable to make progress.
  If a container fails its liveness probe repeatedly, the kubelet restarts the container.
  Liveness probes do not wait for readiness probes to succeed. If you want to wait before executing a liveness probe, you can either define `initialDelaySeconds` or use a
  [startup probe](#startup-probe).

## Readiness probe

Readiness probes determine when a container is ready to accept traffic. This is useful when waiting for an application to perform time-consuming initial tasks that depend on its backing services; for example: establishing network connections, loading files, and warming caches. Readiness probes can also be useful later in the container’s lifecycle, for example, when recovering from temporary faults or overloads.
If the readiness probe returns a failed state, Kubernetes removes the pod from all matching service endpoints.
Readiness probes run on the container during its whole lifecycle.

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
Last modified March 20, 2026 at 12:34 AM PST: [fix overview (09be2414ee)](https://github.com/kubernetes/website/commit/09be2414eea1e520d343e6cbc425e7a110e5d749)

## Related Pages

* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
* [Using RBAC Authorization](./ref-docs-reference-access-authn-authz-rbac.md-docs-reference-access-authn-authz-rbac.md)
## See Also

- [Documentation Index](./COMPASS.md)
