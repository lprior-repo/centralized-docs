---
doc_id: ref/docs-reference-setup-tools-kubeadm.md/docs-reference-setup-tools-kubeadm
chunk_id: ref/docs-reference-setup-tools-kubeadm.md/docs-reference-setup-tools-kubeadm#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 915
summary: ## Table of Contents  - [Kubeadm](#kubeadm)   - [How to install](#how-to-install)   - [What's next](#whats-next)   - [Feedback](#feedback)  ---  # Kubeadm...
---

## Table of Contents

- [Kubeadm](#kubeadm)
  - [How to install](#how-to-install)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

# Kubeadm
![](/images/kubeadm-stacked-color.png)Kubeadm is a tool built to provide `kubeadm init` and `kubeadm join` as best-practice "fast paths" for creating Kubernetes clusters.
kubeadm performs the actions necessary to get a minimum viable cluster up and running. By design, it cares only about bootstrapping, not about provisioning machines. Likewise, installing various nice-to-have addons, like the Kubernetes Dashboard, monitoring solutions, and cloud-specific addons, is not in scope.
Instead, we expect higher-level and more tailored tooling to be built on top of kubeadm, and ideally, using kubeadm as the basis of all deployments will make it easier to create conformant clusters.
## How to install
To install kubeadm, see the [installation guide](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/).
## What's next
* [kubeadm init](/docs/reference/setup-tools/kubeadm/kubeadm-init/) to bootstrap a Kubernetes control-plane node
* [kubeadm join](/docs/reference/setup-tools/kubeadm/kubeadm-join/) to bootstrap a Kubernetes worker node and join it to the cluster
* [kubeadm upgrade](/docs/reference/setup-tools/kubeadm/kubeadm-upgrade/) to upgrade a Kubernetes cluster to a newer version
* [kubeadm config](/docs/reference/setup-tools/kubeadm/kubeadm-config/) if you initialized your cluster using kubeadm v1.7.x or lower, to configure your cluster for `kubeadm upgrade`
* [kubeadm token](/docs/reference/setup-tools/kubeadm/kubeadm-token/) to manage tokens for `kubeadm join`
* [kubeadm reset](/docs/reference/setup-tools/kubeadm/kubeadm-reset/) to revert any changes made to this host by `kubeadm init` or `kubeadm join`
* [kubeadm certs](/docs/reference/setup-tools/kubeadm/kubeadm-certs/) to manage Kubernetes certificates
* [kubeadm kubeconfig](/docs/reference/setup-tools/kubeadm/kubeadm-kubeconfig/) to manage kubeconfig files
* [kubeadm version](/docs/reference/setup-tools/kubeadm/kubeadm-version/) to print the kubeadm version
* [kubeadm alpha](/docs/reference/setup-tools/kubeadm/kubeadm-alpha/) to preview a set of features made available for gathering feedback from the community
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
Last modified July 12, 2023 at 1:25 AM PST: [Revise docs home page (9520b96a61)](https://github.com/kubernetes/website/commit/9520b96a6162d4af841da63227d2a8710596b975)
## Related Pages

- [Implementation details](docs-reference-setup-tools-kubeadm-implementation-details.md)
- [Authenticating with Bootstrap Tokens](docs-reference-access-authn-authz-bootstrap-tokens.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)