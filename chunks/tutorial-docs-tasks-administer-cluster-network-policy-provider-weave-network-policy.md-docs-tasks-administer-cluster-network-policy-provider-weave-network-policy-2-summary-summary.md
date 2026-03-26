---
doc_id: tutorial/docs-tasks-administer-cluster-network-policy-provider-weave-network-policy.md/docs-tasks-administer-cluster-network-policy-provider-weave-network-policy
chunk_id: tutorial/docs-tasks-administer-cluster-network-policy-provider-weave-network-policy.md/docs-tasks-administer-cluster-network-policy-provider-weave-network-policy#2-summary
chunk_level: summary
chunk_type: prose
heading: Install the Weave Net addon
token_count: 114
summary: ## Install the Weave Net addon Follow the [Integrating Kubernetes via the Addon](https://github.com/weaveworks/weave/blob/master/site/kubernetes/kube-addon.md#-installation) guide. The Weave Net...
---

## Install the Weave Net addon
Follow the [Integrating Kubernetes via the Addon](https://github.com/weaveworks/weave/blob/master/site/kubernetes/kube-addon.md#-installation) guide.
The Weave Net addon for Kubernetes comes with a
[Network Policy Controller](https://github.com/weaveworks/weave/blob/master/site/kubernetes/kube-addon.md#network-policy)
that automatically monitors Kubernetes for any NetworkPolicy annotations on all
namespaces and configures `iptables` rules to allow or block traffic as directed by the policies.