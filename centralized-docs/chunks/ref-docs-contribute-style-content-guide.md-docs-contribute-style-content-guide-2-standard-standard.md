---
doc_id: ref/docs-contribute-style-content-guide.md/docs-contribute-style-content-guide
chunk_id: ref/docs-contribute-style-content-guide.md/docs-contribute-style-content-guide#2-standard
chunk_level: standard
chunk_type: prose
heading: What's allowed
token_count: 442
summary: ## Overview Source for the Kubernetes website, including the docs, resides in the [kubernetes/website](https://github.com/kubernetes/website) repository. Located in the...
---

## Overview
Source for the Kubernetes website, including the docs, resides in the
[kubernetes/website](https://github.com/kubernetes/website) repository.
Located in the `kubernetes/website/content/&lt;language\_code&gt;/docs` folder, the
majority of Kubernetes documentation is specific to the [Kubernetes
project](https://github.com/kubernetes/kubernetes).
## What's allowed
Kubernetes docs allow content for third-party projects only when:
* Content documents software in the Kubernetes project
* Content documents software that's out of project but necessary for Kubernetes to function
* Content is canonical on kubernetes.io, or links to canonical content elsewhere### Third party content
Kubernetes documentation includes applied examples of projects in the Kubernetes
project—projects that live in the [kubernetes](https://github.com/kubernetes) and
[kubernetes-sigs](https://github.com/kubernetes-sigs) GitHub organizations.
Links to active content in the Kubernetes project are always allowed.
Kubernetes requires some third party content to function. Examples include container runtimes (containerd, CRI-O, Docker),
[networking policy](/docs/concepts/extend-kubernetes/compute-storage-net/network-plugins/) (CNI plugins),
[Ingress controllers](/docs/concepts/services-networking/ingress-controllers/),
and [logging](/docs/concepts/cluster-administration/logging/).
Docs can link to third-party open source software (OSS) outside the Kubernetes
project only if it's necessary for Kubernetes to function.
### Dual sourced content
Wherever possible, Kubernetes docs link to canonical sources instead of hosting
dual-sourced content.
Dual-sourced content requires double the effort (or more!) to maintain
and grows stale more quickly.
#### Note:
If you're a maintainer for a Kubernetes project and need help hosting your own docs,
ask for help in [#sig-docs on Kubernetes Slack](https://kubernetes.slack.com/messages/C1J0BPD2M/).
### More information
If you have questions about allowed content, join the [Kubernetes Slack](https://slack.k8s.io/) #sig-docs channel and ask!