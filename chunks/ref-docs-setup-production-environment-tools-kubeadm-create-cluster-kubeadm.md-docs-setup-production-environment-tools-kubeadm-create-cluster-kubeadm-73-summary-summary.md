---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#73-summary
chunk_level: summary
chunk_type: prose
heading: Clean up
token_count: 123
summary: ``` `iptables -F &amp;&amp; iptables -t nat -F &amp;&amp; iptables -t mangle -F &amp;&amp; iptables -X ` ``` If you want to reset the IPVS tables, you must run the following command: ``` `ipvsadm -C...
---

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