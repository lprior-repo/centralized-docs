---
doc_id: ref/docs-contribute-style-content-guide.md/docs-contribute-style-content-guide
chunk_id: ref/docs-contribute-style-content-guide.md/docs-contribute-style-content-guide#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 914
summary: ## Table of Contents  - [Documentation Content Guide](#documentation-content-guide)   - [Overview](#overview)   - [What's allowed](#whats-allowed)     - [Dual sourced content](#dual-sourced-content) ...
---

## Table of Contents

- [Documentation Content Guide](#documentation-content-guide)
  - [Overview](#overview)
  - [What's allowed](#whats-allowed)
    - [Dual sourced content](#dual-sourced-content)
      - [Note:](#note)
    - [More information](#more-information)
  - [Feedback](#feedback)

---

# Documentation Content Guide
This page contains guidelines for Kubernetes documentation.
If you have questions about what's allowed, join the #sig-docs channel in
[Kubernetes Slack](https://slack.k8s.io/) and ask!
You can register for Kubernetes Slack at [https://slack.k8s.io/](https://slack.k8s.io/).
For information on creating new content for the Kubernetes
docs, follow the [style guide](/docs/contribute/style/style-guide/).
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
Last modified June 01, 2023 at 9:43 PM PST: [Tweak line wrappings in content-organization.md (1ec7fe8e63)](https://github.com/kubernetes/website/commit/1ec7fe8e63b65c02b29a473b52351af8c6012664)
## Related Pages

- [Advanced contributing](docs-contribute-advanced.md)
- [Validate IPv4/IPv6 dual-stack](docs-tasks-network-validate-dual-stack.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)