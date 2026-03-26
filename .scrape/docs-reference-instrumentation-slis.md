---
url: https://kubernetes.io/docs/reference/instrumentation/slis/
title: Kubernetes Component SLI Metrics
word_count: 378
filtered: true
elements_removed: 0
density_score: 0.92
---

## Table of Contents

- [Kubernetes Component SLI Metrics](#kubernetes-component-sli-metrics)
  - [SLI Metrics](#sli-metrics)
- [TYPE kubernetes\_healthcheck gauge](#type-kuberneteshealthcheck-gauge)
- [TYPE kubernetes\_healthchecks\_total counter](#type-kuberneteshealthcheckstotal-counter)
  - [Using this data](#using-this-data)
  - [Feedback](#feedback)

---

# Kubernetes Component SLI Metrics
High-level indicators for measuring the reliability and performance of Kubernetes components.
FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
By default, Kubernetes 1.35 publishes Service Level Indicator (SLI) metrics
for each Kubernetes component binary. This metric endpoint is exposed on the serving
HTTPS port of each component, at the path `/metrics/slis`. The
`ComponentSLIs` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
defaults to enabled for each Kubernetes component as of v1.27.
## SLI Metrics
With SLI metrics enabled, each Kubernetes component exposes two metrics,
labeled per healthcheck:
* a gauge (which represents the current state of the healthcheck)
* a counter (which records the cumulative counts observed for each healthcheck state)
You can use the metric information to calculate per-component availability statistics.
For example, the API server checks the health of etcd. You can work out and report how
available or unavailable etcd has been - as reported by its client, the API server.
The prometheus gauge data looks like this:
```
`# HELP kubernetes\_healthcheck [ALPHA] This metric records the result of a single healthcheck.
# TYPE kubernetes\_healthcheck gauge
kubernetes\_healthcheck{name="autoregister-completion",type="healthz"} 1
kubernetes\_healthcheck{name="autoregister-completion",type="readyz"} 1
kubernetes\_healthcheck{name="etcd",type="healthz"} 1
kubernetes\_healthcheck{name="etcd",type="readyz"} 1
kubernetes\_healthcheck{name="etcd-readiness",type="readyz"} 1
kubernetes\_healthcheck{name="informer-sync",type="readyz"} 1
kubernetes\_healthcheck{name="log",type="healthz"} 1
kubernetes\_healthcheck{name="log",type="readyz"} 1
kubernetes\_healthcheck{name="ping",type="healthz"} 1
kubernetes\_healthcheck{name="ping",type="readyz"} 1
`
```
While the counter data looks like this:
```
`# HELP kubernetes\_healthchecks\_total [ALPHA] This metric records the results of all healthcheck.
# TYPE kubernetes\_healthchecks\_total counter
kubernetes\_healthchecks\_total{name="autoregister-completion",status="error",type="readyz"} 1
kubernetes\_healthchecks\_total{name="autoregister-completion",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="autoregister-completion",status="success",type="readyz"} 14
kubernetes\_healthchecks\_total{name="etcd",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="etcd",status="success",type="readyz"} 15
kubernetes\_healthchecks\_total{name="etcd-readiness",status="success",type="readyz"} 15
kubernetes\_healthchecks\_total{name="informer-sync",status="error",type="readyz"} 1
kubernetes\_healthchecks\_total{name="informer-sync",status="success",type="readyz"} 14
kubernetes\_healthchecks\_total{name="log",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="log",status="success",type="readyz"} 15
kubernetes\_healthchecks\_total{name="ping",status="success",type="healthz"} 15
kubernetes\_healthchecks\_total{name="ping",status="success",type="readyz"} 15
`
```
## Using this data
The component SLIs metrics endpoint is intended to be scraped at a high frequency. Scraping
at a high frequency means that you end up with greater granularity of the gauge's signal, which
can be then used to calculate SLOs. The `/metrics/slis` endpoint provides the raw data necessary
to calculate an availability SLO for the respective Kubernetes component.
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
Last modified January 07, 2025 at 8:06 PM PST: [Add description for Kubernetes Component SLI Metrics (6c83f61fa5)](https://github.com/kubernetes/website/commit/6c83f61fa5b0ac1e71feb574b98be72791b315c8)
## Related Pages

- [Switching from Polling to CRI Event-based Updates to Container Status](docs-tasks-administer-cluster-switch-to-evented-pleg.md)
- [Understand Pressure Stall Information (PSI) Metrics](docs-reference-instrumentation-understand-psi-metrics.md)
- [Adopting Sidecar Containers](docs-tutorials-configuration-pod-sidecar-containers.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
