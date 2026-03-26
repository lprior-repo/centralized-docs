---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#7-detailed
chunk_level: detailed
chunk_type: prose
heading: Clean up
token_count: 401
summary: ## Clean up If you used disposable servers for your cluster, for testing, you can switch those off and do no further clean up. You can use `kubectl config delete-cluster` to delete your local...
---

## Clean up
If you used disposable servers for your cluster, for testing, you can
switch those off and do no further clean up. You can use
`kubectl config delete-cluster` to delete your local references to the
cluster.
However, if you want to deprovision your cluster more cleanly, you should
first [drain the node](/docs/reference/generated/kubectl/kubectl-commands#drain)
and make sure that the node is empty, then deconfigure the node.
### Remove the node
Talking to the control-plane node with the appropriate credentials, run:
```
`kubectl drain &lt;node name&gt; --delete-emptydir-data --force --ignore-daemonsets
`
```
Before removing the node, reset the state installed by `kubeadm`:
```
`kubeadm reset
`
```
The reset process does not reset or clean up iptables rules or IPVS tables.
If you wish to reset iptables, you must do so manually:
```
`iptables -F &amp;&amp; iptables -t nat -F &amp;&amp; iptables -t mangle -F &amp;&amp; iptables -X
`
```
If you want to reset the IPVS tables, you must run the following command:
```
`ipvsadm -C
`
```
Now remove the node:
```
`kubectl delete node &lt;node name&gt;
`
```
If you wish to start over, run `kubeadm init` or `kubeadm join` with the
appropriate arguments.
### Clean up the control plane
You can use `kubeadm reset` on the control plane host to trigger a best-effort
clean up.
See the [`kubeadm reset`](/docs/reference/setup-tools/kubeadm/kubeadm-reset/)
reference documentation for more information about this subcommand and its
options.