---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 736
summary: # Hardening Guide - Scheduler Configuration Information about how to make the Kubernetes scheduler more secure. The Kubernetes...
---

# Hardening Guide - Scheduler Configuration
Information about how to make the Kubernetes scheduler more secure.
The Kubernetes [scheduler](/docs/reference/command-line-tools-reference/kube-scheduler/) is
one of the critical components of the
[control plane](/docs/reference/glossary/?all=true#term-control-plane).
This document covers how to improve the security posture of the Scheduler.
A misconfigured scheduler can have security implications.
Such a scheduler can target specific nodes and evict the workloads or applications that are sharing the node and its resources.
This can aid an attacker with a [Yo-Yo attack](https://arxiv.org/abs/2105.00542): an attack on a vulnerable autoscaler.
### Scheduler authentication &amp; authorization command line options
When setting up authentication configuration, it should be made sure that
kube-scheduler's authentication remains consistent with kube-api-server's authentication.
If any request has missing authentication headers, the authentication should happen through the kube-api-server
[allowing all authentication to be consistent in the cluster](/docs/tasks/extend-kubernetes/configure-aggregation-layer/#original-request-username-and-group).
* `authentication-kubeconfig`: Make sure to provide a proper kubeconfig so that
the scheduler can retrieve authentication configuration options from the API Server.
This kubeconfig file should be protected with strict file permissions.
* `authentication-tolerate-lookup-failure`: Set this to `false` to make sure
the scheduler *always* looks up its authentication configuration from the API server.
* `authentication-skip-lookup`: Set this to `false` to make sure
the scheduler *always* looks up its authentication configuration from the API server.
* `authorization-always-allow-paths`: These paths should respond with data that is appropriate
for anonymous authorization. Defaults to `/healthz,/readyz,/livez`.
* `profiling`: Set to `false` to disable the profiling endpoints which are provide debugging information
but which should not be enabled on production clusters as they present a risk of denial of service
or information leakage. The `--profiling` argument is deprecated and can now be provided through the
[KubeScheduler DebuggingConfiguration](/docs/reference/config-api/kube-scheduler-config.v1/#DebuggingConfiguration).
Profiling can be disabled through the kube-scheduler config by setting `enableProfiling` to `false`.
* `requestheader-client-ca-file`: Avoid passing this argument.### Scheduler networking command line options
* `bind-address`: In most cases, the kube-scheduler does not need to be externally accessible.
Setting the bind address to `localhost` is a secure practice.
* `permit-address-sharing`: Set this to `false` to disable connection sharing through `SO\_REUSEADDR`.
`SO\_REUSEADDR` can lead to reuse of terminated connections that are in `TIME\_WAIT` state.
* `permit-port-sharing`: Default `false`. Use the default unless you are confident you understand the security implications.### Scheduler TLS command line options
* `tls-cipher-suites`: Always provide a list of preferred cipher suites.
This ensures encryption never happens with insecure cipher suites.## Scheduling configurations for custom schedulers
When using custom schedulers based on the Kubernetes scheduling code, cluster administrators need to be careful with
plugins that use the `queueSort`, `prefilter`, `filter`, or `permit` [extension points](/docs/reference/scheduling/config/#extension-points).
These extension points control various stages of a scheduling process,
and the wrong configuration can impact the kube-scheduler's behavior in your cluster.